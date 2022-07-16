# -*- coding: utf-8 -*-
from sdl2 import *
import ctypes
import OpenGL.GL as gl

import imgui, config
from imgui.integrations.sdl2 import SDL2Renderer



WINDOW_SIZE = (720, 480)
BTN_SIZE = (96, 16)


def impl_pysdl2_init():
    width, height = WINDOW_SIZE
    window_name = "Twitch Pals Config Editor"

    if SDL_Init(SDL_INIT_EVERYTHING) < 0:
        print("Error: SDL could not initialize! SDL Error: " + SDL_GetError().decode("utf-8"))
        exit(1)

    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1)
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24)
    SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8)
    SDL_GL_SetAttribute(SDL_GL_ACCELERATED_VISUAL, 1)
    SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, 1)
    SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, 16)
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG)
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4)
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1)
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE)

    SDL_SetHint(SDL_HINT_MAC_CTRL_CLICK_EMULATE_RIGHT_CLICK, b"1")
    SDL_SetHint(SDL_HINT_VIDEO_HIGHDPI_DISABLED, b"1")

    window = SDL_CreateWindow(window_name.encode('utf-8'),
                              SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                              width, height,
                              SDL_WINDOW_OPENGL)

    if window is None:
        print("Error: Window could not be created! SDL Error: " + SDL_GetError().decode("utf-8"))
        exit(1)

    gl_context = SDL_GL_CreateContext(window)
    if gl_context is None:
        print("Error: Cannot create OpenGL Context! SDL Error: " + SDL_GetError().decode("utf-8"))
        exit(1)

    SDL_GL_MakeCurrent(window, gl_context)
    if SDL_GL_SetSwapInterval(1) < 0:
        print("Warning: Unable to set VSync! SDL Error: " + SDL_GetError().decode("utf-8"))
        exit(1)

    return window, gl_context


def run_cfg_editor(cfg: config.Config):
    cfg_window_sizes = ['[480, 360]', '[720, 480]', '[1080, 720]', '[1280, 1080]']
    # This should work if the config wasn't manually edited
    try:
        cfg_window_size = cfg_window_sizes.index(str(cfg.config['WINDOW_SIZE']))
    except:
        cfg_window_size = 0


    window, gl_context = impl_pysdl2_init()
    imgui.create_context()
    impl = SDL2Renderer(window)

    window_flags = imgui.WINDOW_NO_MOVE | imgui.WINDOW_NO_COLLAPSE | imgui.WINDOW_NO_SAVED_SETTINGS | imgui.WINDOW_NO_TITLE_BAR | imgui.WINDOW_NO_RESIZE

    running = True
    event = SDL_Event()
    while running:
        while SDL_PollEvent(ctypes.byref(event)) != 0:
            if event.type == SDL_QUIT:
                running = False
                cfg.config['WINDOW_SIZE'] = eval(cfg_window_sizes[cfg_window_size])
                config.save_config_json(cfg)
                break
            elif event.type == SDL_WINDOWEVENT_RESIZED:
                imgui.set_window_size(WINDOW_SIZE[0], WINDOW_SIZE[1])
            impl.process_event(event)
        impl.process_inputs()


        imgui.new_frame()
        imgui.set_next_window_size(WINDOW_SIZE[0], WINDOW_SIZE[1])
        imgui.set_next_window_position(0, 0)
        imgui.begin(label='', closable=False, flags=window_flags)

        imgui.text('Configuration')

        _, cfg.config['CHANNEL'] = imgui.input_text(
            label='Twitch Username',
            value=cfg.config['CHANNEL'],
            buffer_length=32,
            flags=imgui.INPUT_TEXT_CHARS_NO_BLANK,
        )

        _, cfg.config['OAUTH_TOKEN'] = imgui.input_text(
            label='Twitch OAUTH',
            value=cfg.config['OAUTH_TOKEN'],
            buffer_length=48,
            flags=imgui.INPUT_TEXT_PASSWORD | imgui.INPUT_TEXT_CHARS_NO_BLANK,
        )

        _, cfg_window_size = imgui.combo(
            label="Window Size",
            current=cfg_window_size,
            items=cfg_window_sizes,
        )

        cfg.config['FPS'], cfg.config['FPS'] = imgui.slider_int(
            "FPS", cfg.config['FPS'], 24, 60
        )

        _, user_limit = imgui.input_text(
            label='User Limit (0 = uncapped)',
            value=str(cfg.config['USER_LIMIT']),
            buffer_length=6,
            flags=imgui.INPUT_TEXT_CHARS_DECIMAL | imgui.INPUT_TEXT_CHARS_NO_BLANK,
        )

        if len(user_limit) > 0 and user_limit.isdigit():
            cfg.config['USER_LIMIT'] = int(user_limit)

        _, cfg.config['SHOW_MESSAGES'] = imgui.checkbox(
            label="Show Messages", state=cfg.config['SHOW_MESSAGES']
        )

        cfg.config['BG_COL'], cfg.config['BG_COL'] = imgui.color_edit3(
            'Background Colour', *cfg.config['BG_COL'], imgui.COLOR_EDIT_FLOAT
        )

        show_capture_list, _ = imgui.collapsing_header("Capture List")

        if show_capture_list:
            for idx in range(len(cfg.config['CAPTURE'])):
                imgui.push_id(str(idx))
                cfg.config['CAPTURE'][idx], cfg.config['CAPTURE'][idx] = imgui.input_text(
                    label='',
                    value=cfg.config['CAPTURE'][idx],
                    buffer_length=32
                )

                imgui.same_line()
                if imgui.button(label="X"):
                    del cfg.config['CAPTURE'][idx]
                imgui.pop_id()
            
            if imgui.button(label="New", width=BTN_SIZE[0], height=BTN_SIZE[1]):
                cfg.config['CAPTURE'].append('Role')

        
        show_filtered_words, _ = imgui.collapsing_header("Filtered Words")

        if show_filtered_words:
            for idx in range(len(cfg.config['FILTERED_WORDS'])):
                imgui.push_id(str(idx))
                cfg.config['FILTERED_WORDS'][idx], cfg.config['FILTERED_WORDS'][idx] = imgui.input_text(
                    label='',
                    value=cfg.config['FILTERED_WORDS'][idx],
                    buffer_length=32
                )

                imgui.same_line()
                if imgui.button(label="X"):
                    del cfg.config['FILTERED_WORDS'][idx]
                imgui.pop_id()
            
            if imgui.button(label="New", width=BTN_SIZE[0], height=BTN_SIZE[1]):
                cfg.config['FILTERED_WORDS'].append('Word')


        show_colour_pallete, _ = imgui.collapsing_header("Colour Palette")

        if show_colour_pallete:
            for idx in range(len(cfg.config['COLOUR_PALETTE'])):
                imgui.push_id(str(idx))
                cfg.config['COLOUR_PALETTE'][idx], cfg.config['COLOUR_PALETTE'][idx] = imgui.color_edit3(
                    '', *cfg.config['COLOUR_PALETTE'][idx][:3], imgui.COLOR_EDIT_FLOAT
                )

                imgui.same_line()
                if imgui.button(label="X"):
                    del cfg.config['COLOUR_PALETTE'][idx]
                imgui.pop_id()

            if imgui.button(label="New", width=BTN_SIZE[0], height=BTN_SIZE[1]):
                cfg.config['COLOUR_PALETTE'].append((255, 255, 255))

        imgui.end()

        gl.glClearColor(0., 0., 0., 1)
        gl.glClear(gl.GL_COLOR_BUFFER_BIT)

        imgui.render()
        impl.render(imgui.get_draw_data())
        SDL_GL_SwapWindow(window)


    impl.shutdown()
    SDL_GL_DeleteContext(gl_context)
    SDL_DestroyWindow(window)
    SDL_Quit()


if __name__ == '__main__':
    run_cfg_editor(config.load_configs())