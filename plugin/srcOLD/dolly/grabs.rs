#![allow(unused_macros)]
use smash::hash40;
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


unsafe extern "C" fn dolly_grab(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, true);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(9.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 2.0, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(11.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
        macros::game_CaptureCutCommon(fighter);
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}
unsafe extern "C" fn dolly_grabdash(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, true);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.6, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(9.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.8, 0.0, 9.0, 3.2, Some(0.0), Some(9.0), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
        macros::game_CaptureCutCommon(fighter);
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}


unsafe extern "C" fn dolly_grabturn(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            GrabModule::set_rebound(fighter.module_accessor, true);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 7.2, -5.0, Some(0.0), Some(7.2), Some(-13.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(fighter, 1, Hash40::new("top"), 1.65, 0.0, 7.2, -3.35, Some(0.0), Some(7.2), Some(-15.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
        }
        macros::game_CaptureCutCommon(fighter);
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
            GrabModule::set_rebound(fighter.module_accessor, false);
        }
}


unsafe extern "C" fn dolly_throwf(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 1.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 1.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
        }
        frame(fighter.lua_state_agent, 23.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 1.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("haver"), /*Damage*/ 1.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
        }
        frame(fighter.lua_state_agent, 37.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 42.0);
        if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ true, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
        }
        frame(fighter.lua_state_agent, 44.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        frame(fighter.lua_state_agent, 50.0);
        if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 7.3, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 2.0, /*Angle*/ 47, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.5), /*Hitlag*/ 0.5, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_POS, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_coin"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_KICK);
        }
        frame(fighter.lua_state_agent, 50.0);
        if macros::is_excute(fighter) {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
}

unsafe extern "C" fn dolly_throwf_effect(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new_raw(0x12147c5df3), Hash40::new("top"), 4, 0, 1, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
        }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
        frame(fighter.lua_state_agent, 42.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
}
unsafe extern "C" fn dolly_sound_throwf(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
        }
        frame(fighter.lua_state_agent, 42.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
        }
}

unsafe extern "C" fn dolly_throwb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 59, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        macros::CHECK_FINISH_CAMERA(fighter, 11, 1);
        //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
        //lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 10.0, y: 0.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}
unsafe extern "C" fn dolly_throwb_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_hit_kick_m"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_down_m_01"));
    }
}

unsafe extern "C" fn dolly_throwb_effect(fighter: &mut L2CAgentBase) {
		frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 1, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new_raw(0x12147c5df3), Hash40::new("top"), -4, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -6, 0, 2, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, -14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        }
}

unsafe extern "C" fn dolly_throwlw(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 32.0);
        for _ in 0..4 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 85, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 4.5, /*X*/ 0.0, /*Y*/ 0.0, /*Z*/ 4.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 85, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 3.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 85, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 4.0, /*Z*/ 2.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
                macros::ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 0.3, /*Angle*/ 85, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 0, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 0.0, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
            }
            wait(fighter.lua_state_agent, 5.0);
        }
        frame(fighter.lua_state_agent, 67.0);  
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.3, /*Angle*/ 82, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 7.5, /*X*/ 0.0, /*Y*/ 2.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.3, /*Angle*/ 82, /*KBG*/ 95, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 5.5, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 0.0, /*X2*/ Some(0.0), /*Y2*/ Some(0.0), /*Z2*/ Some(0.0), /*Hitlag*/ 1.5, /*SDI*/ 0.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_KICK);
        }
}

unsafe extern "C" fn dolly_throwlw_sound(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_dolly_special_h01"));
        }
}

unsafe extern "C" fn dolly_throwlw_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 20, 0, 90, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE, 0.5);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

unsafe extern "C" fn dolly_throwhi(fighter: &mut L2CAgentBase) {
        let rand_val = smash::app::sv_math::rand(hash40("dolly"), 4);
        if rand_val == 0 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 80, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"),true);
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
            frame(fighter.lua_state_agent, 80.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 4.3, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 3.8, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 46, /*Size*/ 3.7, /*X*/ -0.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"),true);
                wait(fighter.lua_state_agent, 13.0);
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
        if rand_val == 1 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 80, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"),true); 
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
            frame(fighter.lua_state_agent, 80.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 4.3, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.8, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 55, /*Size*/ 3.7, /*X*/ -0.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.1, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_fire"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"),true);
                wait(fighter.lua_state_agent, 13.0);
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
        if rand_val == 2 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 80, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"),true); 
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
            frame(fighter.lua_state_agent, 80.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 4.3, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 3.8, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 90, /*FKB*/ 0, /*BKB*/ 56, /*Size*/ 3.7, /*X*/ -0.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.6, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_elec"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_ELEC, /*Type*/ *ATTACK_REGION_PUNCH);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"),true);
                wait(fighter.lua_state_agent, 13.0);
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
        if rand_val == 3 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 80, /*KBG*/ 105, /*FKB*/ 0, /*BKB*/ 70, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                macros::ATTACK_ABS(fighter, /*Kind*/ *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, /*ID*/ 0, /*Damage*/ 1.0, /*Angle*/ 361, /*KBG*/ 100, /*FKB*/ 0, /*BKB*/ 40, /*Hitlag*/ 0.0, /*Unk*/ 1.0, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*Unk*/ 0.0, /*Unk*/ true, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_S, /*SFXType*/ *COLLISION_SOUND_ATTR_NONE, /*Type*/ *ATTACK_REGION_THROW);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"),true); 
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
            frame(fighter.lua_state_agent, 80.0);
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 8.0, /*Angle*/ 80, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 4.3, /*X*/ 4.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.8, /*X*/ 2.0, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
                macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 7.0, /*Angle*/ 80, /*KBG*/ 110, /*FKB*/ 0, /*BKB*/ 40, /*Size*/ 3.7, /*X*/ -0.8, /*Y*/ 0.0, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_FIRE, /*Type*/ *ATTACK_REGION_PUNCH);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"),true);
                wait(fighter.lua_state_agent, 18.0);
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
}

unsafe extern "C" fn dolly_throwhi_effect(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("dolly_attack_arc3"), Hash40::new("dolly_attack_arc3"), Hash40::new("top"), 2, 14, 2, -34.5, 0.4, 105, 0.67, true, *EF_FLIP_YZ, 1);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 24, 2, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 92.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 4.0, 27.0, 0.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn dolly_throwhi_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_sb03_command"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_dolly_special_sb03_command"));
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_sb02_command"));
    }
}

unsafe extern "C" fn dolly_grabattack(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
}

unsafe extern "C" fn dolly_grabattack_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP_FLIP(fighter, Hash40::new("dolly_attack_arc2"), Hash40::new("dolly_attack_arc2"), Hash40::new("top"), -2, 11.5, 0.3, -69, -97, 106, 0.63, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.6);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn dolly_grabattack_sound(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
}
	
pub fn install() {
    Agent::new("dolly")
        .game_acmd("game_catchwaluigi", dolly_grab)
        .game_acmd("game_catchdashwaluigi", dolly_grabdash)
        .game_acmd("game_catchturnwaluigi", dolly_grabturn)
        .game_acmd("game_throwfwaluigi", dolly_throwf)
        .effect_acmd("effect_throwfwaluigi", dolly_throwf_effect)
        .sound_acmd("sound_throwfwaluigi", dolly_sound_throwf)
        .game_acmd("game_throwbwaluigi", dolly_throwb)
        .sound_acmd("sound_throwbwaluigi", dolly_throwb_sound)
        .effect_acmd("effect_throwbwaluigi", dolly_throwb_effect)
        .game_acmd("game_throwlwwaluigi", dolly_throwlw)
        .sound_acmd("sound_throwlwwaluigi", dolly_throwlw_sound)
        .effect_acmd("effect_throwlwwaluigi", dolly_throwlw_effect)
        .game_acmd("game_throwhiwaluigi", dolly_throwhi)
        .effect_acmd("effect_throwhiwaluigi", dolly_throwhi_effect)
        .sound_acmd("sound_throwhiwaluigi", dolly_throwhi_sound)
        .game_acmd("game_catchattackwaluigi", dolly_grabattack)
        .effect_acmd("effect_catchattackwaluigi", dolly_grabattack_effect)
        .sound_acmd("sound_catchattackwaluigi", dolly_grabattack_sound)
        .install();
}