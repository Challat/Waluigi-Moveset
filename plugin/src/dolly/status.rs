use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*
};
use smash::hash40;
use crate::util::*;
use super::*;
use smashline::Priority::*;
use super::super::*;

unsafe extern "C" fn dolly_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
        FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn dolly_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }

        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_S)(fighter);

}

unsafe extern "C" fn dolly_special_sf_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    let fighter_kind = smash::app::utility::get_kind(boma);
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn dolly_special_sf_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }

        return smashline::original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END)(fighter);

}

unsafe extern "C" fn specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    0.into()
}

unsafe extern "C" fn dolly_special_s_command_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR); 
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
    let fighter_kind = smash::app::utility::get_kind(boma);
	StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        true,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S |
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn dolly_special_s_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
            return 1.into();
        }

        return smashline::original_status(Main, fighter, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND)(fighter);

}

//cartwheel jump cancel
pub(crate) fn is_jc(boma: &mut smash::app::BattleObjectModuleAccessor, fighter_kind : i32, status_kind : i32, frame : f32) -> bool {
	unsafe {
        if MotionModule::frame(boma) >= 21.0 {
                //[fighter_kind, status_kind, hit_condition, jc_start, jc_end]
                let jump_cancel = [
                    [*FIGHTER_KIND_DOLLY, *FIGHTER_STATUS_KIND_SPECIAL_LW, 0, -1, -1]
                ];
                for i in &jump_cancel {
                    if fighter_kind == i[0] && status_kind == i[1] {
                        println!("jc status");
                        if i[3] != -1 && i[4] != -1 {
                            if (frame as i32) < i[3] || (frame as i32) >= i[4] {
                                continue;
                            };
                        };
                        if i[2] != 0 {
                            if AttackModule::is_infliction_status(boma, i[2]) {
                                return true;
                            };
                        } else {
                            return true;
                        };
                    };
                };
            };
            return false;
    }
}

//cartwheel jump cancel
pub(crate) fn check_jump(boma: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	unsafe {
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
            return true;
        };
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
            return true;
        };
        if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
            return true;
        };
        return false;
	}
}

//cartwheel jump cancel
unsafe extern "C" fn jump_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {	
		let lua_state = fighter.lua_state_agent;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let fighter_kind = smash::app::utility::get_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let frame = MotionModule::frame(boma);
        if is_jc(boma, fighter_kind, status_kind, frame) && check_jump(boma){
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                };
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                };
            };
    }
}

//slap cancel
unsafe extern "C" fn slap_cancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let stick_x = ControlModule::get_stick_x(boma);
		let stick_y = ControlModule::get_stick_y(boma);
		let frame = MotionModule::frame(boma);
		
        if [hash40("special_lw_special")].contains(&MotionModule::motion_kind(boma)) {
                    if  (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)) {
                            if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
                            } else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                            } else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                            } else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                            } else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
                            } else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                            };
                    };
            };
    };
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
        /*.status(Pre, *FIGHTER_STATUS_KIND_WAIT, dolly_wait_pre)
        .status(Main, *FIGHTER_STATUS_KIND_WAIT, dolly_wait_main)
        .status(Main, *FIGHTER_STATUS_KIND_SQUAT_WAIT, dolly_squatwait_main)
        .status(Main, *FIGHTER_STATUS_KIND_WALK, dolly_walk_main)
        .status(Pre, *FIGHTER_STATUS_KIND_TURN, dolly_turn_pre)
        .status(Pre, *FIGHTER_STATUS_KIND_TURN_DASH, dolly_turndash_pre)
        .status(Init, *FIGHTER_STATUS_KIND_LANDING, dolly_landing_init)
        .status(Init, *FIGHTER_STATUS_KIND_LANDING_LIGHT, dolly_landinglight_init)*/
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, dolly_special_s_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, dolly_special_s_main)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, dolly_special_sf_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END, dolly_special_sf_main)
        .status(Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, dolly_special_s_command_pre)
        .status(Main, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, dolly_special_s_command_main)
        //.on_start(agent_init)
        .on_line(Main, jump_cancel)
        .on_line(Main, slap_cancel)
        
        .install();

}