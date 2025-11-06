use smash::hash40;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::phx::Vector2f;
use std::os::raw::c_int;
use std::os::raw::c_ulong;
use super::*;

//This assumes that the fighter_kind is mewtwo
pub fn is_waluigi(boma: *mut BattleObjectModuleAccessor) -> bool {
    unsafe {
        let color: i32 = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        crate::MARKED_COLORS[color as usize]
    }
}

pub unsafe extern "C" fn is_waluigiarticle(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_MARIO_FIREBALL {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_DOLLY && is_waluigi(owner_boma) {
            return true;
        }
    }
    return false;
}

static mut STATUS_DURATION : [i32; 8] = [0; 8];
static mut MOTION_DURATION : [i32; 8] = [0; 8];
pub static mut SPEED_X : [f32; 8] = [0.0; 8];
pub static mut SPEED_Y : [f32; 8] = [0.0; 8];
pub static mut ACCEL_X : [f32; 8] = [0.0; 8];
pub static mut ACCEL_Y : [f32; 8] = [0.0; 8];
static mut FULL_HOP_ENABLE_DELAY : [i32; 8] = [0; 8];
pub static mut PREV_SCALE : [f32; 8] = [0.0; 8];

//Cstick
pub static mut SUB_STICK: [Vector2f;9] = [Vector2f{x:0.0, y: 0.0};9];


// Transition Hook static muts:
// 0 - Don't change 
// 1 - Force off
// 2 - Force on 
pub static mut CAN_UPB: [i32; 8] = [0; 8];
pub static mut CAN_SIDEB: [i32; 8] = [0; 8];
pub static mut CAN_DOWNB: [i32; 8] = [0; 8];
pub static mut CAN_NEUTRALB: [i32; 8] = [0; 8];
pub static mut CAN_JUMP_SQUAT: [i32; 8] = [0; 8];
pub static mut CAN_DOUBLE_JUMP: [i32; 8] = [0; 8];
pub static mut CAN_CLIFF: [i32; 8] = [0; 8];
pub static mut CAN_ATTACK_AIR: [i32; 8] = [0; 8];
pub static mut CAN_AIRDODGE: [i32; 8] = [0; 8];
pub static mut CAN_RAPID_JAB: [i32; 8] = [0; 8];
pub static mut CAN_JAB: [i32; 8] = [0; 8];
pub static mut CAN_DASH: [i32; 8] = [0; 8];
pub static mut CAN_TURNDASH: [i32; 8] = [0; 8];

//Jab Flags
pub static mut HAS_ENABLE_COMBO_ON: [bool; 8] = [false; 8];
pub static mut HAS_ENABLE_NO_HIT_COMBO_ON: [bool; 8] = [false; 8];
pub static mut HAS_ENABLE_100_ON: [bool; 8] = [false; 8];

//Position and speed
pub(crate) unsafe fn ray_check_pos(boma: &mut smash::app::BattleObjectModuleAccessor, x_distance : f32, y_distance: f32, ignore_plat: bool) -> u64 {
	GroundModule::ray_check(boma, &Vector2f{ x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{ x: x_distance, y: y_distance}, ignore_plat)
}

// Turns off Autoturn for Ryu, Ken, Terry, and Kazuya Wuboy's code
#[skyline::hook(offset = 0x69a6e0)]
unsafe fn autoturn_handler_waluigi(module_accessor: *mut BattleObjectModuleAccessor, some_bool: bool, some_int: i32, some_uint: u32) -> f32 {
    let fighter_kind = smash::app::utility::get_kind(&mut *module_accessor);
	let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if ((color >= 24 && color <= 31)/*goku black*/ || is_waluigi(module_accessor)/*waluigi*/ || (color >= 103 && color <= 110)/*gogeta*/) && fighter_kind == *FIGHTER_KIND_DOLLY {
        return 0.0;
   	}
    original!()(module_accessor, some_bool, some_int, some_uint)
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

/*unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
		
        if fighter_kind == *FIGHTER_KIND_DOLLY && is_waluigi(boma) {
            // Turning off waluigis ability to use command inputs
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
        }
    }
}*/

unsafe extern "C" fn scale(fighter : &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
	    if ModelModule::scale(boma) == WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0) {
                ModelModule::set_scale(boma, 0.94);
                AttackModule::set_attack_scale(boma, 0.95, true);
                GrabModule::set_size_mul(boma, 0.94);
            }
}
}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }

   	Agent::new("dolly")
       .set_costume(costume.to_vec())
		//.on_line(Main, util_update)
		.on_line(Main, scale)
		//.on_init(agent_init)
		.install();
	//skyline::install_hook!(is_enable_transition_term_hook);
	skyline::install_hook!(autoturn_handler_waluigi);
}
