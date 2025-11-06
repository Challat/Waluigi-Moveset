#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
use std::collections::HashMap;

use skyline_web::dialog_ok::DialogOk;
use smash::lib::lua_const::*;
use std::{fs, path::Path};
use smash::hash40;

pub static mut FIGHTER_MANAGER: usize = 0;
pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

mod dolly;
mod util;

pub fn is_on_ryujinx() -> bool {
    unsafe {
        // Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8004000 {
            println!("we are on Ryujinx");
            return true;
        } else {
            println!("we are not on Ryujinx");
            return false;
        }
    }
}

pub fn quick_validate_install() -> bool {
    let mut passed = true;
    //plugin checks
    let has_param_config = Path::new(
        "rom:/skyline/plugins/libparam_config.nro",
    )
    .is_file();
    let has_css_redirector = Path::new(
        "rom:/skyline/plugins/libthe_csk_collection.nro",
    )
    .is_file();
    let has_arcropolis = Path::new(
        "rom:/skyline/plugins/libarcropolis.nro",
    )
    .is_file();
    let has_nro_hook = Path::new(
        "rom:/skyline/plugins/libnro_hook.nro"
    )
    .is_file();
    let has_smashline = Path::new(
        "rom:/skyline/plugins/libsmashline_plugin.nro",
    )
    .is_file();

    if has_param_config {
        println!("libparam_config.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libparam_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libparam_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_css_redirector {
        println!("libthe_csk_collection.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libthe_csk_collection.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libthe_csk_collection.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_arcropolis {
        println!("libarcropolis.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libarcropolis.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libarcropolis.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_nro_hook {
        println!("libnro_hook.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libnro_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libnro_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_smashline {
        println!("libsmashline_plugin.nro is present");
    } else {
        if is_on_ryujinx() {
            println!("libsmashline_plugin.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } else {
            DialogOk::ok("libsmashline_plugin.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }

    passed
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const MARKER_FILE: &str = "waluigi.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/dolly/model/body/c{:02}/{}",
            x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }


    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }

    param_config::disable_kirby_copy(*FIGHTER_KIND_DOLLY, marked_slots.clone());
    param_config::disable_villager_pocket(*FIGHTER_KIND_DOLLY, marked_slots.clone(), 0);

    param_config::update_int_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("wall_jump_type"), 0, 0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("shield_radius"), 0, 11.8));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("landing_attack_air_frame_n"), 0, 8.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("landing_attack_air_frame_f"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("landing_attack_air_frame_b"), 0, 10.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("landing_attack_air_frame_hi"), 0, 9.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("landing_attack_air_frame_lw"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("start_speed_y_mul"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("stick_x_min"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("stick_x_max"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("stick_x_speed_mul_max"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("control_frame"), 2.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("air_speed_y_mul"), 0.692));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("speed_y_mul"), 0.58));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_special_hi"), smash::hash40("lr_stick_x"), 0.2));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("param_private"), smash::hash40("super_special_damage"), 999.0));
    
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("jump_y"), 0, 47.848/27.0));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("dash_speed"), 0, 1.58/1.65));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("walk_accel_mul"), 0, 0.097/0.1));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("walk_accel_add"), 0, 0.243/0.03));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("walk_speed_max"), 0, 0.84/0.85));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("ground_brake"), 0, 0.062/0.111));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("run_accel_mul"), 0, 0.12/0.12));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("run_accel_add"), 0, 0.026/0.04));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("run_speed_max"), 0, 1.67/1.72));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("jump_speed_x"), 0, 0.87/0.8));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("jump_speed_x_mul"), 0, 0.768/0.75));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("jump_speed_x_max"), 0, 2.672/1.1));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("jump_aerial_speed_x_mul"), 0, 1.089/0.8));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("air_accel_y"), 0, 0.09/0.09));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("jump_aerial_y"), 0, 28.248/29.0));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("jump_initial_y"), 0, 28.548/14.85));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("mini_jump_y"), 0, 15.155/15.2));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("air_accel_x_mul"), 0, 0.085/0.05));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("air_accel_x_add"), 0, 0.02/0.01));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("dive_speed_y"), 0, 2.2/2.368));
    param_config::update_attribute_mul_2(*FIGHTER_KIND_DOLLY, marked_slots.clone(), (smash::hash40("weight"), 0, 93.0/108.0));

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    println!("LOWEST: {} - COLOR NUM: {}", lowest_color, color_num);

    the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: smash::hash40("ui_chara_waluigi"),
            fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(0x1226F4B6CD /* Hash40 of fighter_kind_dolly */), 
            fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(0x1226F4B6CD /* Hash40 of fighter_kind_dolly */), 
            ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(0xfd02e9c22 /* Hash40 of ui_series_wario */), 
            fighter_type: the_csk_collection_api::Hash40Type::Overwrite(0x1353795179 /* Hash40 of fighter_type_normal */), 
            alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
            shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x5501F75E9 /* Hash40 of sb-05 */), 
            name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("waluigi")), 
            exhibit_year: the_csk_collection_api::ShortType::Overwrite(1991), 
            exhibit_day_order: the_csk_collection_api::IntType::Overwrite(112501), 
            extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
            ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(0), 
            skill_list_order: the_csk_collection_api::SignedByteType::Overwrite(15), 
            disp_order: the_csk_collection_api::SignedByteType::Optional(Some(34)), 
            save_no: the_csk_collection_api::SignedByteType::Overwrite(78), 
            chara_count: the_csk_collection_api::SignedByteType::Overwrite(1), 
            is_img_ext_skill_page0: the_csk_collection_api::BoolType::Overwrite(true), 
            is_img_ext_skill_page1: the_csk_collection_api::BoolType::Overwrite(true), 
            is_img_ext_skill_page2: the_csk_collection_api::BoolType::Overwrite(false), 
            can_select: the_csk_collection_api::BoolType::Overwrite(true), 
            is_usable_soundtest: the_csk_collection_api::BoolType::Overwrite(false), 
            is_called_pokemon: the_csk_collection_api::BoolType::Overwrite(false), 
            is_mii: the_csk_collection_api::BoolType::Overwrite(false), 
            is_boss: the_csk_collection_api::BoolType::Overwrite(false), 
            is_hidden_boss: the_csk_collection_api::BoolType::Overwrite(false), 
            is_dlc: the_csk_collection_api::BoolType::Overwrite(false), 
            is_patch: the_csk_collection_api::BoolType::Overwrite(false), 
            is_plural_message: the_csk_collection_api::BoolType::Overwrite(false), 
            is_plural_narration: the_csk_collection_api::BoolType::Overwrite(false), 
            is_article: the_csk_collection_api::BoolType::Overwrite(false), 
            has_multiple_face: the_csk_collection_api::BoolType::Overwrite(false), 
            result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
            result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
            result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
            color_num: the_csk_collection_api::UnsignedByteType::Overwrite(color_num as _), 
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(HashMap::from([
                (0x1337FC912E /* Hash40 of characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_waluigi"))), 
                (0x1340FBA1B8 /* Hash40 of characall_label_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x13D9F2F002 /* Hash40 of characall_label_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x13AEF5C094 /* Hash40 of characall_label_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1330915537 /* Hash40 of characall_label_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x13479665A1 /* Hash40 of characall_label_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x13DE9F341B /* Hash40 of characall_label_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x13A998048D /* Hash40 of characall_label_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1B8B13E500 /* Hash40 of characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1BFC14D596 /* Hash40 of characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1B651D842C /* Hash40 of characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1B121AB4BA /* Hash40 of characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1B8C7E2119 /* Hash40 of characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1BFB79118F /* Hash40 of characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1B62704035 /* Hash40 of characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x1B157770A3 /* Hash40 of characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */)), 
                (0x160ab9eb98, the_csk_collection_api::Hash40Type::Overwrite(0xEA827124F /* Hash40 of ui_chara_dolly */)),
            ])), 
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(HashMap::from([
                (0x915C075DE /* Hash40 of c00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9B3B77E6A /* Hash40 of c01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9825F64F7 /* Hash40 of c02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x924286F43 /* Hash40 of c03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9E18F51CD /* Hash40 of c04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x947F85A79 /* Hash40 of c05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9761040E4 /* Hash40 of c06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9D0674B50 /* Hash40 of c07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9E48F9289 /* Hash40 of n00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x942F8993D /* Hash40 of n01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9731083A0 /* Hash40 of n02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9D5678814 /* Hash40 of n03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x910C0B69A /* Hash40 of n04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9B6B7BD2E /* Hash40 of n05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9875FA7B3 /* Hash40 of n06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x92128AC07 /* Hash40 of n07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9F873561A /* Hash40 of c00_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x95E045DAE /* Hash40 of c01_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x96FEC4733 /* Hash40 of c02_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9C99B4C87 /* Hash40 of c03_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x90C3C7209 /* Hash40 of c04_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x9AA4B79BD /* Hash40 of c05_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x99BA36320 /* Hash40 of c06_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (0x93DD46894 /* Hash40 of c07_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
                (smash::hash40("color_start_index"), the_csk_collection_api::UnsignedByteType::Overwrite(lowest_color as _)),
            ])), 
            ..Default::default()
        },
    );

    the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {
        ui_layout_id: smash::hash40("ui_chara_waluigi_00"), 
        ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_waluigi")), 
        chara_color: the_csk_collection_api::UnsignedByteType::Overwrite(0), 
        eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Overwrite(1), 
        eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Overwrite(1), 
        eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Overwrite(0), 
        eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Overwrite(-20.0), 
        eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Overwrite(184.0), 
        eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Overwrite(44.0), 
        eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Overwrite(194.0), 
        eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        eye_flash_info_pos_x: the_csk_collection_api::FloatType::Overwrite(-3.0), 
        eye_flash_info_pos_y: the_csk_collection_api::FloatType::Overwrite(2.0), 
        chara_1_offset_x: the_csk_collection_api::FloatType::Overwrite(20.0), 
        chara_1_offset_y: the_csk_collection_api::FloatType::Overwrite(-65.0), 
        chara_1_scale: the_csk_collection_api::FloatType::Overwrite(1.1), 
        chara_1_1_offset_x: the_csk_collection_api::FloatType::Overwrite(20.0), 
        chara_1_1_offset_y: the_csk_collection_api::FloatType::Overwrite(-60.0), 
        chara_1_1_scale: the_csk_collection_api::FloatType::Overwrite(1.5), 
        chara_1_2_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_2_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_2_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_1_3_offset_x: the_csk_collection_api::FloatType::Overwrite(8.0), 
        chara_1_3_offset_y: the_csk_collection_api::FloatType::Overwrite(-34.0), 
        chara_1_3_scale: the_csk_collection_api::FloatType::Overwrite(1.42), 
        chara_1_4_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_4_offset_y: the_csk_collection_api::FloatType::Overwrite(-40.0), 
        chara_1_4_scale: the_csk_collection_api::FloatType::Overwrite(1.43), 
        chara_1_5_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_5_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_1_5_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_0_offset_x: the_csk_collection_api::FloatType::Overwrite(130.0), 
        chara_3_0_offset_y: the_csk_collection_api::FloatType::Overwrite(-300.0), 
        chara_3_0_scale: the_csk_collection_api::FloatType::Overwrite(0.75), 
        chara_3_1_offset_x: the_csk_collection_api::FloatType::Overwrite(190.0), 
        chara_3_1_offset_y: the_csk_collection_api::FloatType::Overwrite(-300.0), 
        chara_3_1_scale: the_csk_collection_api::FloatType::Overwrite(0.75), 
        chara_3_2_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_3_2_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_3_2_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_3_offset_x: the_csk_collection_api::FloatType::Overwrite(100.0), 
        chara_3_3_offset_y: the_csk_collection_api::FloatType::Overwrite(-200.0), 
        chara_3_3_scale: the_csk_collection_api::FloatType::Overwrite(0.68), 
        chara_3_4_offset_x: the_csk_collection_api::FloatType::Overwrite(32.0), 
        chara_3_4_offset_y: the_csk_collection_api::FloatType::Overwrite(-32.0), 
        chara_3_4_scale: the_csk_collection_api::FloatType::Overwrite(1.2), 
        chara_3_5_offset_x: the_csk_collection_api::FloatType::Overwrite(140.0), 
        chara_3_5_offset_y: the_csk_collection_api::FloatType::Overwrite(-340.0), 
        chara_3_5_scale: the_csk_collection_api::FloatType::Overwrite(0.7), 
        chara_3_6_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_3_6_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_3_6_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_3_7_offset_x: the_csk_collection_api::FloatType::Overwrite(100.0), 
        chara_3_7_offset_y: the_csk_collection_api::FloatType::Overwrite(-240.0), 
        chara_3_7_scale: the_csk_collection_api::FloatType::Overwrite(0.7), 
        chara_5_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_5_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_5_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_select_icon_list_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_7_0_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_7_0_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_7_0_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_7_1_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_7_1_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_7_1_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        chara_0_offset_x: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_0_offset_y: the_csk_collection_api::FloatType::Overwrite(0.0), 
        chara_0_scale: the_csk_collection_api::FloatType::Overwrite(1.0), 
        spirits_eye_visible: the_csk_collection_api::BoolType::Overwrite(false), 
        ..Default::default()
    });

    dolly::install();
    util::install();
}

#[skyline::main(name = "Waluigi_Moveset")]
pub fn main() {
    if !quick_validate_install() {
        return; // don't do anything else since they don't have all dependencies
    }

    unsafe {
        //allows online play
        extern "C" {
            fn allow_ui_chara_hash_online(ui_chara_hash: u64);
        }
        allow_ui_chara_hash_online(0x108658e080);
    }

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }

    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_waluigi");

    the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_zz01_f_waluigi"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_zz01_f_dolly")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_zz01_f_waluigi")),
        ..Default::default()
    });
    
    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_zz01_f_waluigi"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_zz01_f_waluigi")),
        ..Default::default()
    });
    
    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_zz01_f_waluigi"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_zz01_f_waluigi")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..Default::default()
    });
    
    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_zz01_f_waluigi"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("zz01_f_waluigi")),
        ..Default::default()
    });
    
    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("zz01_f_waluigi"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 7659,
        duration_sample: 359424 
    });
    
    the_csk_collection_api::set_fighter_jingle(hash40("ui_chara_waluigi"), "zz01_f_waluigi");
}