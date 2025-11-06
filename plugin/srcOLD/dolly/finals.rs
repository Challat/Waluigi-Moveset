#![allow(unused_macros)]
use smash::hash40;
use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
            *
        },
        lua2cpp::L2CAgentBase,
        lib::lua_const::*,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};
use smash::lua2cpp::L2CFighterCommon;
use super::*;
use crate::util::*;




// Use these for effects
//pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
//pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");

unsafe extern "C" fn fs_frame(fighter: &mut L2CFighterCommon) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let situation_kind = StatusModule::situation_kind(boma);
	let fighter_kind = smash::app::utility::get_kind(boma);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if is_waluigi(boma) {
		if [hash40("final_start")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 303.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			};
		};
		if [hash40("final_air_start")].contains(&MotionModule::motion_kind(boma)) {
			if MotionModule::frame(boma) >= 303.0 {
				StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
			};
		};
	};
}				
			
unsafe extern "C" fn dolly_finalstart(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
	}
	frame(fighter.lua_state_agent, 138.0);
	if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 0.0, 180, 110, 70, 0, 10.0, 0.0, 0.0, 50.0, Some(0.0), Some(80.0), Some(70.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.0, 361, 110, 70, 0, 10.0, 0.0, 0.0, -50.0, Some(0.0), Some(80.0), Some(-70.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 270, 110, 70, 0, 10.0, 0.0, 80.0, 70.0, Some(0.0), Some(80.0), Some(-70.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.0, 90, 110, 70, 0, 10.0, 0.0, 0.0, 70.0, Some(0.0), Some(0.0), Some(-70.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_KICK);
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 5, 0, 5, 40.0, 0.0, 43.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_KICK);
    }
	frame(fighter.lua_state_agent, 229.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	frame(fighter.lua_state_agent, 230.0);
	if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 50.0, 361, 50, 0, 30, 50.0, 0.0, 45.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_KICK);
    }
	frame(fighter.lua_state_agent, 233.0);
	if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
		HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
	}
}
unsafe extern "C" fn dolly_finalstart_effect(fighter: &mut L2CAgentBase) {
    
}
unsafe extern "C" fn dolly_finalstart_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {        
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
	frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {        
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
	frame(fighter.lua_state_agent, 136.0);
    if macros::is_excute(fighter) {        
        macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_wind_loop"));
    }
	frame(fighter.lua_state_agent, 257.0);
    if macros::is_excute(fighter) {        
        macros::STOP_SE(fighter, Hash40::new("se_common_spirits_wind_loop"));
    }
}

pub fn install() {
    Agent::new("dolly")
        .game_acmd("game_finalstartwaluigi", dolly_finalstart)
        .game_acmd("game_finalairstartwaluigi", dolly_finalstart)
        .effect_acmd("effect_finalstartwaluigi", dolly_finalstart_effect)
        .effect_acmd("effect_finalairstartwaluigi", dolly_finalstart_effect)
        .sound_acmd("sound_finalstartwaluigi", dolly_finalstart_sound)
        .sound_acmd("sound_finalairstartwaluigi", dolly_finalstart_sound)
		.on_line(Main, fs_frame)
        .install();
}