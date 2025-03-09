use bitflags::bitflags;
use athena_macros::generate_display;
use crate::entities::roles;

generate_display! {
    #[display(roles::Model)]
    DisplayRole {
        id = i32: base.id
    }
}


bitflags! {
    #[derive(Default)]
    pub struct Permissions: u32 {
        // Administrator permissions
        const ADMIN                     = 1 << 0;
        const MANAGE_ROLES              = 1 << 1;
        const MANAGE_SITE_SETTINGS      = 1 << 2;

        // General team permissions
        const VIEW_ROUNDS               = 1 << 3;
        const MANAGE_ROUNDS             = 1 << 4;
        const MANAGE_PICKS              = 1 << 5;
        const MANAGE_GAMEMODE_PICKS     = 1 << 6;
        const MANAGE_METADATA           = 1 << 7;
        const MANAGE_MODERATION         = 1 << 8;
    }
}
