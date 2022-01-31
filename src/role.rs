use sfml::graphics::Color;


#[derive(Copy, Clone)]
pub enum RoleType {
	VIEWER, VIP, MODERATOR, BROADCASTER
}

pub fn get_colour(role: RoleType) -> Color {
	match role {
		RoleType::VIEWER => Color::WHITE,
		RoleType::MODERATOR => Color::rgb(32, 161, 34),
		RoleType::VIP => Color::rgb(159, 114, 198),
		RoleType::BROADCASTER => Color::rgb(122, 208, 255),
	}
}

pub fn get_roletype(role: String) -> RoleType {
	match role.as_str() {
		"viewers" => RoleType::VIEWER,
		"vips" => RoleType::VIP,
		"moderators" => RoleType::MODERATOR,
		"broadcaster" => RoleType::BROADCASTER,
		_ => RoleType::VIEWER,
	}
}