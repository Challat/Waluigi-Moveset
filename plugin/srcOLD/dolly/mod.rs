use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*
};

mod status;
mod normals;
mod smashes;
mod air;
mod specialn;
mod specials;
mod finals;
mod grabs;
mod taunts;
//mod superspecials;

pub const FIGHTER_DOLLY_GENERATE_ARTICLE_DICEBLOCK: i32 = 0x4;
pub const DICEBLOCK_STATUS_KIND_BREAK: i32 = 1;
pub const DICEBLOCK_STATUS_KIND_DIE: i32 = 2;

pub fn install() {
	status::install();
	normals::install();
	smashes::install();
	air::install();
	specialn::install();
	specials::install();
	finals::install();
	grabs::install();
	taunts::install();
	//superspecials::install();

    smashline::clone_weapon("mario", "fireball", "dolly", "diceblock", true);
    
 }