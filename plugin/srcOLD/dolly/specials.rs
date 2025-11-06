use crate::util::*;
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


pub static mut COPTER_DIR: [i32; 8] = [0; 8];
static mut STICK_DIRECTION : [f32; 8] = [0.0; 8];
static mut DIR_MULT : f32 = 57.295776842880464966688235343549; //Very fun number that turns direction that spits out ControlModule::get_stick_dir(boma) as an angle in degrees
static mut WAS_UPB : [bool; 8] = [false; 8];
static mut HAS_WALL_JUMP : [bool; 8] = [true; 8];
static mut BAN_SIDEB : [bool; 8] = [false; 8];

unsafe extern "C" fn dolly_specialsbstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 12.5, /*Angle*/ 66, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
	frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}


unsafe extern "C" fn dolly_specialsbstart_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
}



unsafe extern "C" fn dolly_specialsbstart_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn dolly_specialsbstart_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_absorption"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.9, false);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_muzzleflash"), Hash40::new("throw"), 0, 0, 0, 360, 0, 0, 1.9, false);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
}



unsafe extern "C" fn dolly_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
        let lua_state = fighter.lua_state_agent;
        let module_accessor = sv_system::battle_object_module_accessor(lua_state);
        //let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
        let situation_kind = StatusModule::situation_kind(boma);
        let fighter_kind = smash::app::utility::get_kind(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        let mut stick_x = ControlModule::get_stick_x(boma);
        let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        stick_x *= PostureModule::lr(boma);

        if fighter_kind == *FIGHTER_KIND_DOLLY && is_waluigi(boma) {
            //removes the ability to use command inputs
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
            
            if (StatusModule::situation_kind(boma) == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK) || (StatusModule::situation_kind(boma) == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_f_start") {
                WAS_UPB[ENTRY_ID] = true;
                BAN_SIDEB[ENTRY_ID] = true;
                if stick_y <= -0.5 {
					GroundModule::pass_floor(boma);
					if ray_check_pos(boma, 0.0, -0.3, false) == 1 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					};
				}
                else {
					GroundModule::clear_pass_floor(boma);
					if ray_check_pos(boma, 0.0, -0.3, true) == 1 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					};
				};
                if MotionModule::frame(boma) >= 19.0 && MotionModule::frame(boma) <= 87.0 {
                    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_end"), 0.0, 1.0, true, 0.0, false, false);
					};
                };
                if MotionModule::frame(boma) >= 88.0 {
                    WAS_UPB[ENTRY_ID] = false;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
                if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) && MotionModule::frame(boma) > 8.0 && MotionModule::frame(boma) < 90.0 && HAS_WALL_JUMP[ENTRY_ID] {
					HAS_WALL_JUMP[ENTRY_ID] = false;
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASSIVE_WALL_JUMP, true);
				};
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_f_end") {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                if stick_y <= -0.5 {
					GroundModule::pass_floor(boma);
					if ray_check_pos(boma, 0.0, -0.3, false) == 1 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					};
				}
                else {
					GroundModule::clear_pass_floor(boma);
					if ray_check_pos(boma, 0.0, -0.3, true) == 1 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					};
				};
                StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                if MotionModule::frame(boma) >= 2.0 && MotionModule::frame(boma) <= 19.0 {
                    macros::SET_SPEED_EX(fighter, 0.4, 1.62, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if MotionModule::frame(boma) >= 20.0 && MotionModule::frame(boma) <= 25.0 {
                    macros::SET_SPEED_EX(fighter, 0.2, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if MotionModule::frame(boma) >= 33.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_f_end2"), 0.0, 1.0, true, 0.0, false, false);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_f_end2") {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                if stick_y <= -0.5 {
					GroundModule::pass_floor(boma);
					if ray_check_pos(boma, 0.0, -0.3, false) == 1 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					};
				} 
                else {
					GroundModule::clear_pass_floor(boma);
					if ray_check_pos(boma, 0.0, -0.3, true) == 1 {
						MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
						StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
					};
				};
                if MotionModule::frame(boma) >= 0.0 {
                    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                    macros::SET_SPEED_EX(fighter, 0.25, -2.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if  AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    MotionModule::set_frame(fighter.module_accessor, 176.0, true);
                    macros::SET_SPEED_EX(fighter, 0.0, 0.9, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
                };
            };
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_f_start") {
                if MotionModule::frame(boma) >= 57.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
                if WAS_UPB[ENTRY_ID] {
                    MotionModule::change_motion(boma, smash::phx::Hash40::new("landing_fall_special"), 0.0, 1.0, false, 0.0, false, false);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    WAS_UPB[ENTRY_ID] = false;
                }
            };
            if ![hash40("special_b_start"),hash40("special_b_attack"),hash40("special_b_attack_w"),hash40("special_b_landing"),hash40("special_air_f_start"),hash40("special_f_start"),hash40("special_f_attack"),hash40("special_f_end"),hash40("landing_fall_special"),hash40("special_air_f_end"),hash40("special_air_f_end2"),hash40("special_air_b_start"),hash40("special_air_b_end"),hash40("special_b_start"),hash40("special_b_attack"),hash40("special_b_attack_w"),hash40("special_b_landing")].contains(&MotionModule::motion_kind(boma)){
                WAS_UPB[ENTRY_ID] = false;
            };
            if BAN_SIDEB[ENTRY_ID] == true && StatusModule::situation_kind(boma) != *SITUATION_KIND_AIR{
                CAN_SIDEB[ENTRY_ID] = 0;
                BAN_SIDEB[ENTRY_ID] = false;
                HAS_WALL_JUMP[ENTRY_ID] = true;
            };
            if BAN_SIDEB[ENTRY_ID] == true {
                CAN_SIDEB[ENTRY_ID] = 1;
            } else {
                CAN_SIDEB[ENTRY_ID] = 0;
            };
            //removes upthrow loop sound effect if it doesnt reach the stop frame on the sound script
            if MotionModule::motion_kind(fighter.module_accessor) != hash40("throw_hi") {
                macros::STOP_SE(fighter, Hash40::new("se_dolly_special_sb03_command"));
            };
        };
    }	
}


unsafe extern "C" fn dolly_specialairsfend(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		}
}
unsafe extern "C" fn dolly_specialairsfend_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_splash"), Hash40::new("top"), 0, 0, 2.5, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
}
unsafe extern "C" fn dolly_specialairsfend_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_water_hit_ll"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appear_01"));
    }
}

unsafe extern "C" fn dolly_specialairsfend2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("head"), /*Damage*/ 7.0, /*Angle*/ 266, /*KBG*/ 88, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
}
unsafe extern "C" fn dolly_specialairsfend2_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..30 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 30.0, 0, 90, 0, 0, 1.3, true, *EF_FLIP_YZ, 0.3);
            //macros::LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
}
unsafe extern "C" fn dolly_specialairsfend2_sound(fighter: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn dolly_specialsfstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 12.5, /*Angle*/ 66, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 7.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_bind"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}


unsafe extern "C" fn dolly_specialsfstart_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_absorption"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.9, false);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_muzzleflash"), Hash40::new("throw"), 0, 0, 0, 360, 0, 0, 1.9, false);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.6);
    }
}


unsafe extern "C" fn dolly_specialsfstart_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
}


unsafe extern "C" fn dolly_specialsfstart_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}


unsafe extern "C" fn dolly_specialsfattack(fighter: &mut L2CAgentBase) {
        
}
unsafe extern "C" fn dolly_specialsfattack_sound(fighter: &mut L2CAgentBase) {
    
}
	
unsafe extern "C" fn dolly_specialsfattack_effect(fighter: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn dolly_specialairsfstart(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
            JostleModule::set_status(fighter.module_accessor, false);
			    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"),false);
			    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"),false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"),false);
			    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES); 
            }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 6.8, /*Angle*/ 61, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 6.8, /*Angle*/ 61, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 1.0, 1.22, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 1.2, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 1.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 1.9, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 4.0, /*Angle*/ 91, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 64, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("throw"), /*Damage*/ 4.0, /*Angle*/ 91, /*KBG*/ 125, /*FKB*/ 0, /*BKB*/ 65, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.4, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_ENERGY);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }
        frame(fighter.lua_state_agent, 48.0);
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 1.7, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(fighter.lua_state_agent, 55.0);
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 1.6, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        frame(fighter.lua_state_agent, 70.0);
        if macros::is_excute(fighter) {
            macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.5);
        }
        frame(fighter.lua_state_agent, 105.0);
        if macros::is_excute(fighter) {
            macros::SET_SPEED_EX(fighter, 1.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            AttackModule::clear_all(fighter.module_accessor);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
        }
}

unsafe extern "C" fn dolly_specialairsfstart_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 13, 6.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 13, 6.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 11, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 11, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
    frame(fighter.lua_state_agent, 66.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 7.5, 8.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_water_landing"), Hash40::new("top"), 0, 7.5, 8.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.1, 0, 0.6);
    }
}

unsafe extern "C" fn dolly_specialairsfstart_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_water_hit_m"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_water_hit_m"));
    }
    frame(fighter.lua_state_agent, 66.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_water_hit_m"));
    }
}

unsafe extern "C" fn dolly_specialsfaattack(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 1.0);
		if macros::is_excute(fighter) {
		    macros::SET_SPEED_EX(fighter, 0.00, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_RUN_BRAKE, true);
		}
		frame(fighter.lua_state_agent, 44.0);
		if macros::is_excute(fighter) {
		    macros::SET_SPEED_EX(fighter, 0.00, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_RUN_BRAKE, true);
        }
}

unsafe extern "C" fn dolly_specialsfaattack_sound(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND);
        if(WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND)){
            if macros::is_excute(fighter) {
                macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_sf03_command_s"));
            }
            else{
            //WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND);
            if(!(WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND))){
                if macros::is_excute(fighter) {
                    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_sf03_command_l"));
                }
                else{
                if macros::is_excute(fighter) {
                    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_sf02_s"));
                }
                else{
                if macros::is_excute(fighter) {
                    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_sf02_l"));
                }
            }
        }
    }
}
}
}
}

unsafe extern "C" fn dolly_specialhi1(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::SA_SET(fighter, *SITUATION_KIND_AIR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        }
        frame(fighter.lua_state_agent, 8.0);
	    if macros::is_excute(fighter) {
            let x_speed = PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor) * 0.75;
		    macros::SET_SPEED_EX(fighter, x_speed, 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
		    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
		    MotionModule::set_rate(fighter.module_accessor, 1.2 );
	    }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
            macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 98, /*KBG*/ 70, /*FKB*/ 110, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 75, /*KBG*/ 70, /*FKB*/ 110, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -2.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
            AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
            WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(6.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(19.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        else{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            }
        }
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        else{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            }
        }

        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        else{
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            }
        }

        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 33.0);
        if macros::is_excute(fighter) {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            MotionModule::set_rate(fighter.module_accessor, 1.3);
        }
}

unsafe extern "C" fn dolly_specialhi1_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 0, 1.2, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 142.0/255.0, 81.0/255.0, 1.0/255.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 0, 0, 0.75, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 90, 0, 1.15, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 90, 0, 0.95, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 142.0/255.0, 81.0/255.0, 1.0/255.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 90, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 1, 0, 0, 0, 1.9, true, 0.5);
    }
}

unsafe extern "C" fn dolly_specialhi1_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_dolly_special_h01"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_s"));
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_s"));
            //macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h01"));
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_s"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_s"));
    }
    else{
    frame(fighter.lua_state_agent, 0.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_l"));
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_l"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h02"));
    }
}
frame(fighter.lua_state_agent, 14.0);
if macros::is_excute(fighter) {
    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_l"));
}
frame(fighter.lua_state_agent, 20.0);
if macros::is_excute(fighter) {
    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_l"));
}
frame(fighter.lua_state_agent, 23.0);
if macros::is_excute(fighter) {
    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_l"));
}
}

unsafe extern "C" fn dolly_specialairhi1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        let x_speed = PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor) * 0.75;
        macros::SET_SPEED_EX(fighter, x_speed, 1.3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(fighter.module_accessor, 1.2 );
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 98, /*KBG*/ 70, /*FKB*/ 110, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 6.5, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(2.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 75, /*KBG*/ 70, /*FKB*/ 110, /*BKB*/ 30, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ -2.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-3.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(6.0), /*Hitlag*/ 1.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 17.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(19.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(10.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 4.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(4.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 6.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(0.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 20, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
    }

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 30, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
    }

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
	wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
	    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
    }

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
	wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
	    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
    }

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
	wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
	    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
    }

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
	wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
	    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 70, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 11, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 30, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    else{
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 90, /*KBG*/ 14, /*FKB*/ 40, /*BKB*/ 10, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(9.0), /*Z2*/ Some(-1.5), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        }
    }

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 367, /*KBG*/ 14, /*FKB*/ 50, /*BKB*/ 11, /*Size*/ 3.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ -4.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(-6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
	wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.3, /*Angle*/ 367, /*KBG*/ 64, /*FKB*/ 50, /*BKB*/ 51, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(3.5), /*Z2*/ Some(6.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.3, /*Angle*/ 70, /*KBG*/ 64, /*FKB*/ 30, /*BKB*/ 51, /*Size*/ 3.5, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(18.5), /*Z2*/ Some(1.0), /*Hitlag*/ 0.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

unsafe extern "C" fn dolly_specialairhi1_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 0, 1.2, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 142.0/255.0, 81.0/255.0, 1.0/255.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 0, 0, 0.75, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 90, 0, 1.15, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 8.5, 0, 180, 90, 0, 0.95, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 142.0/255.0, 81.0/255.0, 1.0/255.0);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.0, 0, 180, 90, 0, 0.7, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.4, 0.1, 0.8);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 1, 0, 0, 0, 1.9, true, 0.5);
    }
}

unsafe extern "C" fn dolly_specialairhi1_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_dolly_special_h01"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_s"));
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_s"));
            //macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h01"));
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_s"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_s"));
    }
    else{
    frame(fighter.lua_state_agent, 0.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_l"));
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_l"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h02"));
    }
}
frame(fighter.lua_state_agent, 14.0);
if macros::is_excute(fighter) {
    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_l"));
}
frame(fighter.lua_state_agent, 20.0);
if macros::is_excute(fighter) {
    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_l"));
}
frame(fighter.lua_state_agent, 23.0);
if macros::is_excute(fighter) {
    macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_l"));
}
}

/////////////////////////////////////////////////////////////////////////

unsafe extern "C" fn dolly_specialairlw(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {

    }
}
unsafe extern "C" fn dolly_specialairlw_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -2.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn dolly_specialairlw_sound(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        
    }
}

unsafe extern "C" fn dolly_speciallwstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
     		ArticleModule::generate_article(fighter.module_accessor, FIGHTER_DOLLY_GENERATE_ARTICLE_DICEBLOCK, false, -1);   
    }
}
unsafe extern "C" fn dolly_speciallwstart_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn dolly_speciallwstart_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_win02_02"));
    }
}

unsafe extern "C" fn dolly_specialairlwstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        
    }
}
unsafe extern "C" fn dolly_specialairlwstart_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -0.5, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn dolly_specialairlwstart_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_win02_02"));
    }
}
unsafe extern "C" fn dolly_speciallwshield(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        
    }
}
unsafe extern "C" fn dolly_speciallwshield_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_masterball"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.55, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -2, 27, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 180, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 180, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 1.5, 0, 0, 0, 180, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn dolly_speciallwshield_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_system_fp_level_up"));
    }
}

unsafe extern "C" fn dolly_speciallwattack1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.0, /*Angle*/ 37, /*KBG*/ 80, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 10.0, /*Z*/ 8.5, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwattack1_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.13, 1.55, 0.0, 0.65);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_absorption"), Hash40::new("top"), -4, 10, 8.5, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.35);
    }
}
unsafe extern "C" fn dolly_speciallwattack1_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_smash_l01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
}

unsafe extern "C" fn dolly_speciallwattack2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 2.5, /*Angle*/ 80, /*KBG*/ 25, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwattack2_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        macros::FLASH(fighter, 0.13, 1.55, 0.0, 0.65);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_absorption"), Hash40::new("top"), -4, 12, 10, 0, 0, 0, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.35);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
    }
}
unsafe extern "C" fn dolly_speciallwattack2_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_smash_l02"));
    }	
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_s01"));
    }
}

unsafe extern "C" fn dolly_speciallwattack3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 5.5, /*Angle*/ 66, /*KBG*/ 96, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 12.0, /*Z*/ 10.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_paralyze"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwattack3_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        macros::FLASH(fighter, 0.13, 1.55, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_masterball"), Hash40::new("top"), 0, 3.5, 1, 0, 0, 0, 0.4, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("handl"), 4, 0, 0, 0, 0, 0, 0.35, true);
    }
}
unsafe extern "C" fn dolly_speciallwattack3_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_smash_h02"));
    }	
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n05"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

unsafe extern "C" fn dolly_speciallwattackspecial1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 4.0, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_BAT, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 5.0, /*Angle*/ 66, /*KBG*/ 80, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwattackspecial1_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("handr"), 0, -2, 0, 0, 0, 0, 0.45, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_fire_fly"), false, false);
    }
}
unsafe extern "C" fn dolly_speciallwattackspecial1_sound(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

unsafe extern "C" fn dolly_speciallwattackspecial2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("footl"), /*Damage*/ 7.0, /*Angle*/ 33, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("kneel"), /*Damage*/ 4.0, /*Angle*/ 33, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwattackspecial2_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_absorption"), false, false);
        macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 7.0, 0, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
}
unsafe extern "C" fn dolly_speciallwattackspecial2_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_attackair_b01"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_b02"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

unsafe extern "C" fn dolly_speciallwjump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.25);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
}
unsafe extern "C" fn dolly_speciallwjump_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.0, 0.09, 2.55, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        //macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn dolly_speciallwjump_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
}

unsafe extern "C" fn dolly_speciallwspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 5.5, /*Angle*/ 90, /*KBG*/ 96, /*FKB*/ 60, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_KICK, /*Type*/ *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwspecial_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 0, 2, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
}
unsafe extern "C" fn dolly_speciallwspecial_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {        
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
}

unsafe extern "C" fn dolly_speciallwattackair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 5.0, 361, 20, 0, 50, 6.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 5.0, 361, 20, 0, 50, 5.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 5.0, 50, 20, 0, 60, 6.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), 5.0, 40, 20, 0, 60, 5.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footr"), 7.0, 33, 104, 0, 40, 6.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 33, 104, 0, 40, 5.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwattackair_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.13, 1.55, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footl"), 0, 2, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("footr"), 0, 2, 0, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}
    unsafe extern "C" fn dolly_speciallwattackair_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_l01"));
    }
	wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
}

unsafe extern "C" fn dolly_speciallwspecialair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.6);
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 80, 0, 5.0, 0.0, 4.0, 6.0, Some(0.0), Some(17.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 100, 80, 0, 5.0, 0.0, 4.0, -6.0, Some(0.0), Some(17.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_turn"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}
unsafe extern "C" fn dolly_speciallwspecialair_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
}
unsafe extern "C" fn dolly_speciallwspecialair_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_attackdash01"));
    }
}

unsafe extern "C" fn dolly_speciallwjumpair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 4.0);
	if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
}
unsafe extern "C" fn dolly_speciallwjumpair_effect(fighter: &mut L2CAgentBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0.0, 0.09, 2.55, 0.55);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ColorBlendModule::off_flash(boma, false);
        COL_NORMAL(fighter.lua_state_agent);
    }
}
unsafe extern "C" fn dolly_speciallwjumpair_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_stage_mariobros_09"));
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_stage_mariobros_09"));
        }
}
unsafe extern "C" fn dolly_frame_mischief(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
        let lua_state = fighter.lua_state_agent;
        let module_accessor = sv_system::battle_object_module_accessor(lua_state);
        let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
        let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let stick_y = ControlModule::get_stick_y(boma);
        STICK_DIRECTION[ENTRY_ID] = ControlModule::get_stick_dir(boma) * DIR_MULT;
            
        if fighter_kind == *FIGHTER_KIND_DOLLY && is_waluigi(boma) {
            //mischief step transitions
            if [hash40("special_lw_start")].contains(&MotionModule::motion_kind(boma)) { //grounded mischief step
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 12.0 && MotionModule::frame(boma) <= 22.0 {
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND) { //grounded shield (moonwalk)
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_shield"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND) { //grounded attack (destruction dance)
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_attack1"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND) { //grounded jump (cartwheel)
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_jump"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND) { //grounded special (slap)
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_special"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
				};
                if MotionModule::frame(boma) > 22.0 { //no input pressed
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
            };
            if [hash40("special_air_lw_start")].contains(&MotionModule::motion_kind(boma)) { //aerial mischief step
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 12.0 && MotionModule::frame(boma) <= 22.0 {
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR) { //aerial shield (moonwalk)
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_shield"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR) { //aerial attack (trick kick)
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_air_attack"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR) { //aerial attack (trick kick)
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_air_special"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) && (StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR) { //aerial jump (copter)
                        if ControlModule::get_stick_x(boma) >= -0.2 && ControlModule::get_stick_x(boma) <= 0.2 && ControlModule::get_stick_y(boma) >= -0.2 && ControlModule::get_stick_y(boma) <= 0.2 {
                            STICK_DIRECTION[ENTRY_ID] = 361.0;
                        } else if STICK_DIRECTION[ENTRY_ID] <= -67.5 {
                            STICK_DIRECTION[ENTRY_ID] *= -1.0;
                        };
                        if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x < 0.0 {
                            COPTER_DIR[ENTRY_ID] = 1;
                        }
                        else if STICK_DIRECTION[ENTRY_ID] >= 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y < 0.0 {
                            COPTER_DIR[ENTRY_ID] = 2;
                        }
                        else if STICK_DIRECTION[ENTRY_ID] >= -67.5 && STICK_DIRECTION[ENTRY_ID] < -22.5 && stick_x > 0.0 {
                            COPTER_DIR[ENTRY_ID] = 3;
                        }
                        else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x < 0.0 {
                            COPTER_DIR[ENTRY_ID] = 4;
                        }
                        else if STICK_DIRECTION[ENTRY_ID] == 361.0 {
                            COPTER_DIR[ENTRY_ID] = 5;
                        }
                        else if STICK_DIRECTION[ENTRY_ID] >= -22.5 && STICK_DIRECTION[ENTRY_ID] <= 22.5 && stick_x > 0.0 {
                            COPTER_DIR[ENTRY_ID] = 6;
                        }
                        else if STICK_DIRECTION[ENTRY_ID] > 22.5 && STICK_DIRECTION[ENTRY_ID] <= 67.5 && stick_x < 0.0 {
                            COPTER_DIR[ENTRY_ID] = 7;
                        }
                        else if STICK_DIRECTION[ENTRY_ID] > 67.5 && STICK_DIRECTION[ENTRY_ID] <= 90.0 && stick_y > 0.0 {
                            COPTER_DIR[ENTRY_ID] = 8;
                        }
                        else  {
                            COPTER_DIR[ENTRY_ID] = 9;
                        };
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_air_jump"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    };
				};
                if MotionModule::frame(boma) > 22.0 { //no input pressed
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };

            //moonwalk shit
            if [hash40("special_lw_shield")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 58.0 {
                    if (StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                    if (StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                    };
                };
            };

            //destruction dance shit
            if [hash40("special_lw_attack1")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 28.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
                if MotionModule::frame(boma) >= 20.0 && MotionModule::frame(boma) < 28.0 {
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)) {
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_attack2"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) {
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_attack_special2"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                };
            };
            if [hash40("special_lw_attack2")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 48.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
                if MotionModule::frame(boma) >= 29.0 && MotionModule::frame(boma) < 48.0 {
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)) {
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_attack3"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                    if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) {
                        MotionModule::change_motion(boma, smash::phx::Hash40::new("special_lw_attack_special1"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                    };
                };
            };
            if [hash40("special_lw_attack3")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 42.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
            };
            if [hash40("special_lw_attack_special1")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 37.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
            };
            if [hash40("special_lw_attack_special2")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 37.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
            };

            //cartwheel shit
            if [hash40("special_lw_jump")].contains(&MotionModule::motion_kind(boma)) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                //StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 20.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };

            //slap shit
            if [hash40("special_lw_special")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 35.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                };
            };

            //trick kick shit
            if [hash40("special_lw_air_attack")].contains(&MotionModule::motion_kind(boma)) {
                if MotionModule::frame(boma) >= 1.0 {
                    macros::SET_SPEED_EX(fighter, 0, 0.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if MotionModule::frame(boma) >= 15.0 {
                    macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if MotionModule::frame(boma) >= 35.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };

            //trick spin shit
            if [hash40("special_lw_air_special")].contains(&MotionModule::motion_kind(boma)) {
                macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if MotionModule::frame(boma) >= 86.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };

            //copter shit
            if [hash40("special_lw_air_jump")].contains(&MotionModule::motion_kind(boma)) {
                if COPTER_DIR[ENTRY_ID] == 1 {
                    macros::SET_SPEED_EX(fighter, -1.5, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else if COPTER_DIR[ENTRY_ID] == 2 {
                    macros::SET_SPEED_EX(fighter, 0, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else if COPTER_DIR[ENTRY_ID] == 3 {
                    macros::SET_SPEED_EX(fighter, 1.5, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else if COPTER_DIR[ENTRY_ID] == 4 {
                    macros::SET_SPEED_EX(fighter, -1.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); 
                }
                else if COPTER_DIR[ENTRY_ID] == 5 {
                    macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN); //no direction
                }
                else if COPTER_DIR[ENTRY_ID] == 6 {
                    macros::SET_SPEED_EX(fighter, 1.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else if COPTER_DIR[ENTRY_ID] == 7 {
                    macros::SET_SPEED_EX(fighter, -1.5, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else if COPTER_DIR[ENTRY_ID] == 8 {
                    macros::SET_SPEED_EX(fighter, 0, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else if COPTER_DIR[ENTRY_ID] == 9 {
                    macros::SET_SPEED_EX(fighter, 1.5, 1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                if MotionModule::frame(boma) >= 26.0 {
                    macros::SET_SPEED_EX(fighter, 0, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if MotionModule::frame(boma) >= 28.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
                    StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_AIR), true);
                };
            };
        };
    }	
}

pub fn install() {
	Agent::new("dolly")
        .game_acmd("game_specialsbstartwaluigi", dolly_specialsbstart)
        .sound_acmd("sound_specialsbstartwaluigi", dolly_specialsbstart_sound)
        .expression_acmd("expression_specialsbstartwaluigi", dolly_specialsbstart_expression)
        .effect_acmd("effect_specialsbstartwaluigi", dolly_specialsbstart_effect)
        .game_acmd("game_specialairsfendwaluigi", dolly_specialairsfend)
        .effect_acmd("effect_specialairsfendwaluigi", dolly_specialairsfend_effect)
        .sound_acmd("sound_specialairsfendwaluigi", dolly_specialairsfend_sound)
        .game_acmd("game_specialairsfend2", dolly_specialairsfend2)
        .effect_acmd("effect_specialairsfend2", dolly_specialairsfend2_effect)
        .game_acmd("game_specialsfstartwaluigi", dolly_specialsfstart)
        .effect_acmd("effect_specialsfstartwaluigi", dolly_specialsfstart_effect)
        .sound_acmd("sound_specialsfstartwaluigi", dolly_specialsfstart_sound)
        .expression_acmd("expression_specialsfstartwaluigi", dolly_specialsfstart_expression)
        .game_acmd("game_specialairsfstartwaluigi", dolly_specialairsfstart)
        .effect_acmd("effect_specialairsfstartwaluigi", dolly_specialairsfstart_effect)
        .sound_acmd("sound_specialairsfstartwaluigi", dolly_specialairsfstart_sound)
        .game_acmd("game_specialairsfattackwaluigi", dolly_specialsfaattack)
        .sound_acmd("sound_specialairsfattackwaluigi", dolly_specialsfaattack_sound)
        .game_acmd("game_specialhi1waluigi", dolly_specialhi1)
        .effect_acmd("effect_specialhi1waluigi", dolly_specialhi1_effect)
        .sound_acmd("sound_specialhi1waluigi", dolly_specialhi1_sound)
        .game_acmd("game_specialairhi1waluigi", dolly_specialairhi1)
        .effect_acmd("effect_specialairhi1waluigi", dolly_specialairhi1_effect)
        .sound_acmd("sound_specialairhi1waluigi", dolly_specialairhi1_sound)
        .effect_acmd("effect_speciallwstartwaluigi", dolly_speciallwstart_effect)
        .effect_acmd("effect_specialairlwstartwaluigi", dolly_specialairlwstart_effect)
        .sound_acmd("sound_speciallwstartwaluigi", dolly_speciallwstart_sound)
        .sound_acmd("sound_specialairlwstartwaluigi", dolly_specialairlwstart_sound)
        .effect_acmd("effect_speciallwshield", dolly_speciallwshield_effect)
        .sound_acmd("sound_speciallwshield", dolly_speciallwshield_sound)
        .game_acmd("game_speciallwattack1", dolly_speciallwattack1)
        .effect_acmd("effect_speciallwattack1", dolly_speciallwattack1_effect)
        .sound_acmd("sound_speciallwattack1", dolly_speciallwattack1_sound)
        .game_acmd("game_speciallwattack2", dolly_speciallwattack2)
        .effect_acmd("effect_speciallwattack2", dolly_speciallwattack2_effect)
        .sound_acmd("sound_speciallwattack2", dolly_speciallwattack2_sound)
        .game_acmd("game_speciallwattack3", dolly_speciallwattack3)
        .effect_acmd("effect_speciallwattack3", dolly_speciallwattack3_effect)
        .sound_acmd("sound_speciallwattack3", dolly_speciallwattack3_sound)
        .game_acmd("game_speciallwattackspecial1", dolly_speciallwattackspecial1)
        .effect_acmd("effect_speciallwattackspecial1", dolly_speciallwattackspecial1_effect)
        .sound_acmd("sound_speciallwattackspecial1", dolly_speciallwattackspecial1_sound)
        .game_acmd("game_speciallwattackspecial2", dolly_speciallwattackspecial2)
        .effect_acmd("effect_speciallwattackspecial2", dolly_speciallwattackspecial2_effect)
        .sound_acmd("sound_speciallwattackspecial2", dolly_speciallwattackspecial2_sound)
        .game_acmd("game_speciallwjump", dolly_speciallwjump)
        .effect_acmd("effect_speciallwjump", dolly_speciallwjump_effect)
        .sound_acmd("sound_speciallwjump", dolly_speciallwjump_sound)
        .game_acmd("game_speciallwspecial", dolly_speciallwspecial)
        .effect_acmd("effect_speciallwspecial", dolly_speciallwspecial_effect)
        .sound_acmd("sound_speciallwspecial", dolly_speciallwspecial_sound)
        .game_acmd("game_speciallwattackair", dolly_speciallwattackair)
        .effect_acmd("effect_speciallwattackair", dolly_speciallwattackair_effect)
        .sound_acmd("sound_speciallwattackair", dolly_speciallwattackair_sound)
        .game_acmd("game_speciallwspecialair", dolly_speciallwspecialair)
        .effect_acmd("effect_speciallwspecialair", dolly_speciallwspecialair_effect)
        .sound_acmd("sound_speciallwspecialair", dolly_speciallwspecialair_sound)
        .game_acmd("game_speciallwjumpair", dolly_speciallwjumpair)
        .effect_acmd("effect_speciallwjumpair", dolly_speciallwjumpair_effect)
        .sound_acmd("sound_speciallwjumpair", dolly_speciallwjumpair_sound)
        
        .on_line(Main, dolly_frame)
        .on_line(Main, dolly_frame_mischief)
        .install();
}