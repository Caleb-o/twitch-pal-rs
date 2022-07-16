from dataclasses import dataclass
from typing import Any
import json, os


CONFIG_PATH = 'data/config.json'
REQUEST_PATH = 'data/remove_requests.json'


@dataclass
class Config:
	config: dict[str, Any]
	remove_requsts: list[str]
	display_size: tuple[int, int]


def save_config_json(cfg: Config):
	try:
		with open(CONFIG_PATH, 'w') as out_file:
			json.dump(cfg.config, out_file, indent=4)
	except:
		print('Could not write to config')


def save_rr_json(cfg: Config):
	try:
		with open(REQUEST_PATH, 'w') as out_file:
			json.dump(cfg.remove_requsts, out_file, indent=4)
	except:
		print('Could not write to remove requests')


def save_json(cfg: Config):
	save_config_json(cfg)
	save_rr_json(cfg)


def load_configs() -> Config:
	# Default configuration for twitch stuff
	cfg = Config({
		'CHANNEL': 'User',
		'OAUTH_TOKEN': 'None',
		'WINDOW_SIZE': (1280, 720),
		'BG_COL': (40, 40, 40),
		'FPS': 24,
		'USER_LIMIT': 0,
		'SHOW_MESSAGES': True,
		'CAPTURE': ['broadcaster','admins','moderators','viewers','vips'],
		'FILTERED_WORDS': [],
		'COLOUR_PALETTE': [
			(217, 215, 241), (255, 253, 222), (231, 251, 190), (255, 203, 203), (211, 228, 205), (173, 194, 169), 
			(153, 167, 153), (198, 213, 126), (213, 126, 126), (162, 205, 205), (255, 225, 175) 
		]
	}, [], (640, 360))


	# Save the json if it doesn't exist
	if not os.path.isfile(CONFIG_PATH):
		save_config_json(cfg)
	if not os.path.isfile('remove_requests.json'):
		save_rr_json(cfg)

	# FIXME: Make these individual, rather than both at once.
	#		 Not having one will actually skip this, so stuff might break
	if os.path.isfile(CONFIG_PATH) and os.path.isfile(REQUEST_PATH):
		# Try to load the config
		try:
			with open(CONFIG_PATH) as in_file:
				cfg.config = json.load(in_file)

			with open(REQUEST_PATH) as in_file:
				cfg.remove_requsts = json.load(in_file)
		except:
			print('Could not load file')


	# Set display size from config
	size = cfg.config['WINDOW_SIZE']
	cfg.display_size = (size[0]//2, size[1]//2)

	return cfg