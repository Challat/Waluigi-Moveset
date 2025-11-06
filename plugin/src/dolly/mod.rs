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

pub static mut FIGHTER_DOLLY_GENERATE_ARTICLE_DICEBLOCK: i32 = 4;
pub const DICEBLOCK_STATUS_KIND_BREAK: i32 = 1;
//pub const DICEBLOCK_STATUS_KIND_DIE: i32 = 2;

//Collision Log
#[repr(C)]
pub struct CollisionLog {
    pub next: *mut CollisionLog,
    pub end: *mut CollisionLog,
    pub location: Vector3f,
    pub padding_0: u32,
    pub padding_1: u32,
    pub opponent_battle_object_id: u32,
    pub padding_2: [u8;7],
    pub collision_kind: u8,
    pub receiver_part_id: u8,
    pub collider_part_id: u8,
    pub receiver_id: u8,
    pub collider_id: u8,
    pub padding_3: [u8;10]
}
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

    unsafe {
        FIGHTER_DOLLY_GENERATE_ARTICLE_DICEBLOCK += smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "dolly", "diceblock", false);
    }
 }