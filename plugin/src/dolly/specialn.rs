#![allow(unused_macros)]
use smash::hash40;
use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
			Weapon,
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
use crate::util::*;
use super::*;
use smashline::Priority::*;
use super::super::*;

const semitone_0_up : f32 = 1.0;
const semitone_1_up : f32 = 1.05946;
const semitone_2_up : f32 = 1.1225;
const semitone_3_up : f32 = 1.18921;
const semitone_4_up : f32 = 1.26;
const semitone_5_up : f32 = 1.33484;
const semitone_6_up : f32 = 1.41421;
const semitone_7_up : f32 = 1.49831;
const semitone_8_up : f32 = 1.58740;
const semitone_9_up : f32 = 1.68179;
const semitone_10_up : f32 = 1.78180;
const semitone_11_up : f32 = 1.88775;
const semitone_12_up : f32 = 2.0;
const semitone_13_up : f32 = 2.11893;
const semitone_14_up : f32 = 2.24492;
const semitone_15_up : f32 = 2.37841;
const semitone_16_up : f32 = 2.51984;

pub static mut RANDOM_NUM: [i32; 8] = [0; 8];
pub static mut RANDOM_NUM_SELECTED: [bool; 8] = [false; 8];
pub static mut ALREADY_BLOCK: [bool; 8] = [false; 8];
pub static mut DICEBLOCK_FRAME: [i32; 8] = [0; 8];
pub static mut ROLLED_NUMBER: [i32; 8] = [0; 8];
pub static mut LIFE_FRAME: [i32; 8] = [0; 8];
pub static mut REMOVE_EXIST: [i32; 8] = [0; 8];
pub static mut STATUS_FRAME: [i32; 8] = [0; 8];

unsafe extern "C" fn dolly_frame_nb(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let lua_state = fighter.lua_state_agent;
        let module_accessor = sv_system::battle_object_module_accessor(lua_state);
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = smash::app::utility::get_kind(boma);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		
		if [hash40("special_n"),hash40("special_air_n")].contains(&MotionModule::motion_kind(boma)) {
				if MotionModule::frame(boma) <= 1.0 {
					DICEBLOCK_FRAME[ENTRY_ID] = 0;
					RANDOM_NUM[ENTRY_ID] = smash::app::sv_math::rand(hash40("dolly"), 10);
				};
				if MotionModule::frame(boma) <= 2.0 {
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dolly_Kart_Glider_VIS_O_OBJShape"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10_trans"), false);
				};
				if MotionModule::frame(boma) < 18.0 {
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), true);
					if RANDOM_NUM[ENTRY_ID] == 0  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 1 {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 2  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 3  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 4  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 5  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 6 {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 7  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 8  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), true);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
					else if RANDOM_NUM[ENTRY_ID] == 9  {
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
						ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), true);
						ALREADY_BLOCK[ENTRY_ID] = true;
					}
				};
				if MotionModule::frame(boma) >= 18.0 {
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("dolly_Kart_Glider_VIS_O_OBJShape"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10_trans"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
					ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
				};
			};
	}
}


unsafe extern "C" fn waluigi_grounded_neutral_special_acmd(fighter: &mut L2CAgentBase) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	
	frame(fighter.lua_state_agent, 19.0);
	if macros::is_excute(fighter) {
		ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_DICEBLOCK, false, -1);
	}
}

unsafe extern "C" fn waluigi_grounded_neutral_special_effect(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0.0, 30.0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
	}
	frame(fighter.lua_state_agent, 2.0);
	if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("handl"), 2.0, 3, -9.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
	}
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
	}
}

unsafe extern "C" fn waluigi_neutral_special_sound(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 0.0);
	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_ok"));
	}
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_superspecial_hit_critical"));
	}
	frame(fighter.lua_state_agent, 11.0);
	if macros::is_excute(fighter) {
		macros::STOP_SE(fighter, Hash40::new("se_dolly_superspecial_hit_critical"));
	}
}

unsafe extern "C" fn diceblock(fighter: &mut L2CAgentBase) {
	let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let life = WorkModule::get_param_int(fighter.module_accessor, hash40("diceblock"), hash40("life"));

    LIFE_FRAME[ENTRY_ID] = life;
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
    }
}
unsafe extern "C" fn diceblock_eff(fighter: &mut L2CAgentBase) {
	
}
unsafe extern "C" fn diceblock_snd(fighter: &mut L2CAgentBase) {

}

unsafe extern "C" fn diceblock_break(fighter: &mut L2CAgentBase) {
	let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let dice_num = ROLLED_NUMBER[ENTRY_ID];

	if macros::is_excute(fighter) {
        AttackModule::clear_all(diceblock_boma);
        match (dice_num + 1) {
            // healing
            1 => {
                macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 0.0, 0, 0, 0, 0, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PSI);
            }
            // weak coin
            2 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 20, 0, 30, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PSI);
            }
            // paralyze
            3 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 60, 50, 0, 0, 10.3, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            }
            // freeze
            4 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 80, 60, 0, 50, 10.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_PSI);
            }
            // reverse hit
            5 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PSI);
            }
            // water / ink
            6 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 80, 61, 0, 22, 10.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_PSI);
                AttackModule::set_ink_value(diceblock_boma, 0, 150.0);
            }
            // jackpot
            7 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 128, 0, 66, 10.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PSI);
            }
            // explosion
            8 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 50, 85, 0, 65, 10.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
            }
            // poison
            9 => {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 40, 0, 25, 10.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
                AttackModule::set_poison_param(diceblock_boma, 0, 361, 45, 1.5, false);
            }
            // multihit
            /* 10 */ _ => {
                for i in 0..7 {
                    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 366, 0, 100, 40, 10.3, 0.0, 0.0, 0.0, None, None, None, 3.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
                    wait(fighter.lua_state_agent, 3.0);
                    if macros::is_excute(fighter) {
                        AttackModule::clear_all(diceblock_boma);
                    }
                    wait(fighter.lua_state_agent, 3.0);
                }
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 90, 0, 100, 40, 10.3, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
            }
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(diceblock_boma);
        REMOVE_EXIST[ENTRY_ID] = 1;
    }
}
unsafe extern "C" fn diceblock_break_eff(fighter: &mut L2CAgentBase) {
	let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let dice_num = ROLLED_NUMBER[ENTRY_ID];

    if macros::is_excute(fighter) {
		macros::EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		macros::EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("top"), 0, 4, -1, 0, 0, 180, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_pokemon_out"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        if dice_num == 0 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_1"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 1 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 2 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_3"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 3 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_4"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 4 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_5"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 5 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_6"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 6 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_7"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 7 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_8"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 8 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_9"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        }
        else if dice_num == 9 {
            macros::EFFECT(fighter, Hash40::new("waluigi_diceblock_num_10"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe extern "C" fn diceblock_break_snd(fighter: &mut L2CAgentBase) {
	let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let dice_num = ROLLED_NUMBER[ENTRY_ID];

	if macros::is_excute(fighter) {
		macros::PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_success"));
		if dice_num == 0 {
			macros::PLAY_SE(fighter, Hash40::new("se_common_lifeup"));
		}
		else if dice_num == 1 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_2_up);
		}
		else if dice_num == 2 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_4_up);
		}
		else if dice_num == 3 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_5_up);
		}
		else if dice_num == 4 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_7_up);
		}
		else if dice_num == 5 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_9_up);
		}
		else if dice_num == 6 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_11_up);
		}
		else if dice_num == 7 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_12_up);
		}
		else if dice_num == 8 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_14_up);
		}
		else if dice_num == 9 {
			macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
			SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_dolly_special_n02"), semitone_16_up);
		}
	}
}

unsafe extern "C" fn regular_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32
    );

    0.into()
}

unsafe extern "C" fn regular_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	
	MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);

    let rng = sv_math::rand(hash40("fighter"), 10);
    //println!("Dice roll: {}", rng);
    ROLLED_NUMBER[ENTRY_ID] = rng;

    ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("frame_dice"), true);
    for i in 1..=10 {
        ModelModule::set_mesh_visibility(
            weapon.module_accessor,
            Hash40::new(format!("num_dice_{}", i).as_str()),
            i == rng + 1
        );
    }

    weapon.fastshift(L2CValue::Ptr(regular_main_loop as *const () as _))
}

unsafe extern "C" fn regular_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	
	let life = LIFE_FRAME[ENTRY_ID];
    if life <= 0 {
        weapon.change_status(DICEBLOCK_STATUS_KIND_BREAK.into(), false.into());
        return 0.into();
    }

    LIFE_FRAME[ENTRY_ID] -= 1;

    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        weapon.change_status(DICEBLOCK_STATUS_KIND_BREAK.into(), false.into());
    }

    if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        return 0.into();
    }
    
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    weapon.pop_lua_stack(1);

    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_bound"), -1);
    MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_SOUND, Hash40::new("sound_bound"), -1);
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        weapon.change_status(DICEBLOCK_STATUS_KIND_BREAK.into(), false.into());
        return 0.into();
    }
       
    0.into()
}

unsafe extern "C" fn break_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ATTACK as i32
    );

    0.into()
}

unsafe extern "C" fn break_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	VisibilityModule::set_whole(weapon.module_accessor, false);
	sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("break"), 1.0, 1.0, false, 0.0, false, false);

	weapon.fastshift(L2CValue::Ptr(break_main_loop as *const () as _))
}

pub unsafe extern "C" fn break_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("frame_dice"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_1"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_2"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_3"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_4"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_5"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_6"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_7"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_8"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_9"), false);
	ModelModule::set_mesh_visibility(weapon.module_accessor, Hash40::new("num_dice_10"), false);

	if STATUS_FRAME[ENTRY_ID] > 1 && REMOVE_EXIST[ENTRY_ID] == 1 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
	STATUS_FRAME[ENTRY_ID] += 1;

	0.into()
}

pub unsafe extern "C" fn break_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let diceblock_boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    let boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(diceblock_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	
	ROLLED_NUMBER[ENTRY_ID] = 0;
	REMOVE_EXIST[ENTRY_ID] = 0;
	STATUS_FRAME[ENTRY_ID] = 0;

    0.into()
}

#[skyline::from_offset(0x33bcd10)]
unsafe extern "C" fn normal_weapon_on_attack_waluigi(vtable: u64, weapon: *mut Weapon, arg3: u64, log: CollisionLog);

unsafe extern "C" fn fireball_on_attack_waluigi(vtable: u64, weapon: *mut smash::app::Weapon, arg3: u64, log: CollisionLog) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let owner_kind = smash::app::utility::get_kind(&mut *owner_boma);
    
	if owner_kind == *FIGHTER_KIND_DOLLY
	&& is_waluigi(owner_boma) {
		*(weapon as *mut bool).add(0x90) = true;
		if log.collider_part_id == 1 {
			//*(weapon as *mut bool).add(0x90) = true;
			let opponent_boma = &mut *(sv_battle_object::module_accessor(log.opponent_battle_object_id));
			DamageModule::add_damage(opponent_boma, -15.0, 0);
		}
    }

    normal_weapon_on_attack_waluigi(vtable, weapon, arg3, log)
}

unsafe extern "C" fn diceblock_bound_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
	}
}
unsafe extern "C" fn diceblock_bound_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
		macros::PLAY_SE(agent, Hash40::new("se_common_landing_glass"));
	}
}

pub const FIREBALL_ON_ATTACK_PTR: usize = 0x51e2c80;

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
	.game_acmd("game_specialnwaluigi", waluigi_grounded_neutral_special_acmd, Default)
	.effect_acmd("effect_specialnwaluigi", waluigi_grounded_neutral_special_effect, Default)
	.sound_acmd("sound_specialnwaluigi", waluigi_neutral_special_sound, Default)
	.sound_acmd("sound_specialairnwaluigi", waluigi_neutral_special_sound, Default)
	.game_acmd("game_specialairnwaluigi", waluigi_grounded_neutral_special_acmd, Default)
	.effect_acmd("effect_specialairnwaluigi", waluigi_grounded_neutral_special_effect, Default)
        
		.on_line(Main, dolly_frame_nb)
		.install();

	Agent::new("dolly_diceblock")
    .set_costume(costume.to_vec())
	.effect_acmd("effect_bound", diceblock_bound_eff, Default)
    .sound_acmd("sound_bound", diceblock_bound_snd, Default)
	.game_acmd("game_regular", diceblock, Default)
	.effect_acmd("effect_regular", diceblock_eff, Default)
	.sound_acmd("sound_regular", diceblock_snd, Default)
	.game_acmd("game_break", diceblock_break, Default)
	.effect_acmd("effect_break", diceblock_break_eff, Default)
	.sound_acmd("sound_break", diceblock_break_snd, Default)
		
	.status(Pre, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, regular_pre)
	.status(Main, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, regular_main)

    .status(Pre, DICEBLOCK_STATUS_KIND_BREAK, break_pre)
	.status(Main, DICEBLOCK_STATUS_KIND_BREAK, break_main)
    .status(End, DICEBLOCK_STATUS_KIND_BREAK, break_end)
		
		//.on_line(Main, diceblock_frame)
		.install();
	
	skyline::patching::Patch::in_text(FIREBALL_ON_ATTACK_PTR).data((fireball_on_attack_waluigi as *const ()));
}