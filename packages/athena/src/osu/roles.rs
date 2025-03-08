use athena_macros::generate_display;

use crate::entities::roles;

generate_display! {
    #[display(roles::Model)]
    DisplayRole {
        id = i32: base.id
    }
}