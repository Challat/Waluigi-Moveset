#![allow(unused_macros)]use 
smash::hash40;
use smash::lib::lua_const::*;
use smash_script::macros::is_excute as other_is_excute;
use smash::app::AttackHeight;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash::app::lua_bind::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_battle_object::module_accessor;
use smash::app::{self, sv_information};
use smash::app::*;
#[allow(unused_macros)]
use smash::app::utility::get_category;
use smash::app::sv_battle_object::entry_id;
use smash::app::lua_bind::ModelModule::set_mesh_visibility;
use smash_script::macros::*;
use smash::lib::L2CAgent;
use smash::lib::L2CValue;
use smash::app::stage::get_stage_id;
use smash::lib::lua_const::StageID::CampaignMap;
use smash::lib::lua_const::StageID::SP_Edit;
use smash::app::boss_private::is_multi_play_mode;
use std::{thread, time};
use smash::app::smashball::is_training_mode;
use std::ffi::CStr;
use std::ffi::CString;
use std::sync::atomic::{ Ordering, AtomicBool };
use smash::app::sv_animcmd::*;
use smash::lua2cpp::L2CFighterAnimcmdGameCommon;
use {
    smash::{
		app::{lua_bind::*, *},
		app::ItemKind,
		//app::sv_battle_object::module_accessor,
		//smash_script::macros::is_excute as other_is_excute,
		//smash::app::AttackHeight,
        //app::lua_bind::*,
        //lua2cpp::L2CFighterCommon,
        //app::sv_battle_object::module_accessor,
		app::lua_bind::ControlModule,
		//lib::LuaConst,
        //app::*,
        // app::utility::get_category,
        // app::sv_battle_object::entry_id,
        //app::lua_bind::ModelModule::set_mesh_visibility,
        //lib::L2CAgent,
        //lib::L2CValue,
        //app::stage::get_stage_id,
        //lib::lua_const::StageID::CampaignMap,
        //lib::lua_const::StageID::SP_Edit,
        //app::boss_private::is_multi_play_mode,
        //app::smashball::is_training_mode,
		phx::Vector3f,
		//app::sv_math,
        // app::{lua_bind::*, sv_animcmd::*, *},
		//app::{self, sv_information},
		//smash_script::macros::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};


unsafe extern "C" fn dolly_appealhil(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 3.0);
	    if macros::is_excute(fighter) {
		    macros::FT_MOTION_RATE(fighter, 1.62);
        }
}

unsafe extern "C" fn dolly_appealhileffect(fighter: &mut L2CAgentBase) {
        //original!(fighter)}
	
}

unsafe extern "C" fn dolly_appealhileffect_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new_raw(0x136a368b1c));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_dolly_appeal01_02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
}

unsafe extern "C" fn dolly_appealhil_expression(fighter: &mut L2CAgentBase) {
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 53.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
}

unsafe extern "C" fn dolly_appealhir(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::FT_MOTION_RATE(fighter, 1.62);
        }
}

unsafe extern "C" fn dolly_appealhir_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new_raw(0x136a368b1c));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_dolly_appeal01_02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
}

unsafe extern "C" fn dolly_appealhir_expression(fighter: &mut L2CAgentBase) {
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 53.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
}


unsafe extern "C" fn dolly_appeallwl(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_dolly_win02"));
        }
}
unsafe extern "C" fn dolly_appeallwl_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 85.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l03"));
    }
}

unsafe extern "C" fn dolly_appeallwl_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 64.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 90.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn dolly_appeallwr(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_dolly_win02"));
        }
}

unsafe extern "C" fn dolly_appeallwr_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 85.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l03"));
    }
}

unsafe extern "C" fn dolly_appeallwr_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 64.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 90.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
unsafe extern "C" fn dolly_entryr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
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
	}
}

unsafe extern "C" fn dolly_entryr_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn dolly_entryr_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appear01"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appear02"));
    }
    frame(fighter.lua_state_agent, 84.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appear03"));
    }
}

unsafe extern "C" fn dolly_entryr_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
}

unsafe extern "C" fn dolly_entryl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
	if macros::is_excute(fighter) {
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
    }
}

unsafe extern "C" fn dolly_entryl_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn dolly_entryl_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appear01"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appear02"));
    }
    frame(fighter.lua_state_agent, 84.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_appear03"));
    }
}

unsafe extern "C" fn dolly_entryl_expression(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
}

pub fn install() {
    Agent::new("dolly")
        .game_acmd("game_appealhilwaluigi", dolly_appealhil)
        .sound_acmd("sound_appealhilwaluigi", dolly_appealhileffect_sound)
        .expression_acmd("expression_appealhilwaluigi", dolly_appealhil_expression)
        .sound_acmd("sound_appealhirwaluigi", dolly_appealhir)
        .game_acmd("game_appealhirwaluigi", dolly_appealhir_sound)
        .expression_acmd("expression_appealhirwaluigi", dolly_appealhir_expression)
        .game_acmd("game_appeallwlwaluigi", dolly_appeallwl)
        .sound_acmd("sound_appeallwlwaluigi", dolly_appeallwl_sound)
        .expression_acmd("expression_appeallwlwaluigi", dolly_appeallwl_expression)
        .game_acmd("game_appeallwrwaluigi", dolly_appeallwr)
        .sound_acmd("sound_appeallwrwaluigi", dolly_appeallwr_sound)
        .expression_acmd("expression_appeallwrwaluigi", dolly_appeallwr_expression)
        .game_acmd("game_entryrwaluigi", dolly_entryr)
        .effect_acmd("effect_entryrwaluigi", dolly_entryr_effect)
        .sound_acmd("sound_entryrwaluigi", dolly_entryr_sound)
        .expression_acmd("expression_entryrwaluigi", dolly_entryr_expression)
        .game_acmd("game_entrylwaluigi", dolly_entryl)
        .effect_acmd("effect_entrylwaluigi", dolly_entryl_effect)
        .sound_acmd("sound_entrylwaluigi", dolly_entryl_sound)
        .expression_acmd("expression_entrylwaluigi", dolly_entryl_expression)
        .install();
}