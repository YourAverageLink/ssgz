use core::ffi::c_void;

use crate::{game::{actor::{AcObjBase, ActorID, Base}, collision::Acch, enemy::{AcEBc, AcEnBase}, is_valid_game_ptr},  system::math::{Vec3f, Vec3s}};

#[repr(C)]
pub struct ActorLink {
    pub obj_base: AcObjBase,
    pub pad0: [u8; 0x2324],
    pub acch: Acch,
    pub pad1:          [u8; 0x1A94],
    pub stamina_amount: u32,
    // More after
}

impl ActorLink {
    pub fn position(&mut self) -> &mut Vec3f {
        &mut self.obj_base.ac_base.position
    }

    pub fn rotation(&mut self) -> &mut Vec3s {
        &mut self.obj_base.ac_base.rotation
    }

    pub fn get_targeted_actor(&self) -> Option<&mut AcEnBase> {
        let ptr = unsafe { ActorLink__getTargetedActor(self) };
        if !is_valid_game_ptr(ptr) {
            return None;
        }
        unsafe { ptr.as_mut() }
    }
    
    pub fn get_targeted_bokoblin(&self) -> Option<&mut AcEBc> {
        if let Some(enemy) = self.get_targeted_actor() {
            if enemy.obj_base.ac_base.base.profile_name == ActorID::E_BC {
                let boko_ptr = (enemy as *mut AcEnBase).cast::<AcEBc>();
                return Some(unsafe {&mut *boko_ptr});
            }
        }

        None
    }

    pub fn get_riding_actor(&self) -> Option<&mut AcObjBase> {
        let ptr = unsafe { ActorLink__getRidingActor(self) };
        if !is_valid_game_ptr(ptr) {
            return None;
        }
        unsafe { ptr.as_mut() }
    }
}

#[repr(C)]
pub struct ActorLinkOld {
    pub base_base:      Base,
    pub vtable:         u32,
    pub obj_base_pad0:  [u8; 0x54],
    pub angle:          Vec3s,
    pub pad:            [u8; 2],
    pub pos:            Vec3f,
    pub obj_base_pad:   [u8; 0x78], // 0x144 - (0x64 + 0x5C + 0xC)
    pub forward_speed:   f32,
    pub forward_accel:   f32,
    pub forward_max_speed: f32,
    pub velocity:       Vec3f,
    pub pad01:          [u8; 0x433C], // 0x4498 - 0x15C
    pub stamina_amount: u32,
    // More after
}
extern "C" {
    static LINK_PTR: *mut ActorLink;
    fn checkXZDistanceFromLink(actor: *const c_void, distance: f32) -> bool;
    fn ActorLink__setPosRot(player: *mut ActorLink, pos: *const Vec3f, angle: *const Vec3s, force: bool, unk1: u32, unk2: u32);
    fn ActorLink__getTargetedActor(player: *const ActorLink) -> *mut AcEnBase;
    fn ActorLink__getRidingActor(player: *const ActorLink) -> *mut AcObjBase;
}

pub fn as_mut() -> Option<&'static mut ActorLink> {
    unsafe {
        if !is_valid_game_ptr(LINK_PTR) {
            return None;
        }
        LINK_PTR.as_mut()
    }
}

pub fn force_set_link_pos_rot(pos: &Vec3f, angle: &Vec3s) {
    unsafe {
        if let Some(link) = as_mut() {
            let link_ptr = link as *mut ActorLink;
            let pos_ptr = pos as *const Vec3f;
            let angle_ptr = angle as *const Vec3s;
            ActorLink__setPosRot(link_ptr, pos_ptr, angle_ptr, true, 1, 0);
        }
    }
}
