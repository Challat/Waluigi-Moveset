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
use smashline::Priority::*;

use super::super::*;


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

unsafe extern "C" fn dolly_win1_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 74.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
    frame(fighter.lua_state_agent, 103.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_landing01"));
    }
    frame(fighter.lua_state_agent, 127.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_dolly_win01"));
    }
}

unsafe extern "C" fn dolly_win2_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_dolly_attack01"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_dolly_win01"));
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_landing01"));
    }
}

unsafe extern "C" fn dolly_win3_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_step_left_l"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_step_left_l"));
    }
    frame(fighter.lua_state_agent, 104.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_down01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_dolly_appeal02"));
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
    .game_acmd("game_appealhilwaluigi", dolly_appealhil, Default)
    .sound_acmd("sound_appealhilwaluigi", dolly_appealhileffect_sound, Default)
    .expression_acmd("expression_appealhilwaluigi", dolly_appealhil_expression, Default)
    .sound_acmd("sound_appealhirwaluigi", dolly_appealhir, Default)
    .game_acmd("game_appealhirwaluigi", dolly_appealhir_sound, Default)
    .expression_acmd("expression_appealhirwaluigi", dolly_appealhir_expression, Default)
    .game_acmd("game_appeallwlwaluigi", dolly_appeallwl, Default)
    .sound_acmd("sound_appeallwlwaluigi", dolly_appeallwl_sound, Default)
    .expression_acmd("expression_appeallwlwaluigi", dolly_appeallwl_expression, Default)
    .game_acmd("game_appeallwrwaluigi", dolly_appeallwr, Default)
    .sound_acmd("sound_appeallwrwaluigi", dolly_appeallwr_sound, Default)
    .expression_acmd("expression_appeallwrwaluigi", dolly_appeallwr_expression, Default)
    .game_acmd("game_entryrwaluigi", dolly_entryr, Default)
    .effect_acmd("effect_entryrwaluigi", dolly_entryr_effect, Default)
    .sound_acmd("sound_entryrwaluigi", dolly_entryr_sound, Default)
    .expression_acmd("expression_entryrwaluigi", dolly_entryr_expression, Default)
    .game_acmd("game_entrylwaluigi", dolly_entryl, Default)
    .effect_acmd("effect_entrylwaluigi", dolly_entryl_effect, Default)
    .sound_acmd("sound_entrylwaluigi", dolly_entryl_sound, Default)
    .expression_acmd("expression_entrylwaluigi", dolly_entryl_expression, Default)
    .sound_acmd("sound_win1waluigi", dolly_win1_sound, Default)
    .sound_acmd("sound_win2waluigi", dolly_win2_sound, Default)
    .sound_acmd("sound_win3waluigi", dolly_win3_sound, Default)
        .install();
}