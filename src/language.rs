use crate::*;

use libc::strncmp;

#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_count(mut self_0: *const TSLanguage) -> uint32_t {
    return (*self_0).symbol_count.wrapping_add((*self_0).alias_count);
}
/* *
 * Get the ABI version number for this language. This version number is used
 * to ensure that languages were generated by a compatible version of
 * Tree-sitter.
 *
 * See also `ts_parser_set_language`.
 */
#[no_mangle]
pub unsafe extern "C" fn ts_language_version(mut self_0: *const TSLanguage) -> uint32_t {
    return (*self_0).version;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_count(mut self_0: *const TSLanguage) -> uint32_t {
    if (*self_0).version >= 10 as libc::c_int as libc::c_uint {
        return (*self_0).field_count;
    } else {
        return 0 as libc::c_int as uint32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_table_entry(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
    mut symbol: TSSymbol,
    mut result: *mut TableEntry,
) {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
        || symbol as libc::c_int
            == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        (*result).action_count = 0 as libc::c_int as uint32_t;
        (*result).is_reusable = 0 as libc::c_int != 0;
        (*result).actions = 0 as *const TSParseAction
    } else {
        assert!((symbol as libc::c_uint) < (*self_0).token_count);
        let mut action_index: uint32_t = ts_language_lookup(self_0, state, symbol) as uint32_t;
        let mut entry: *const TSParseActionEntry =
            &*(*self_0).parse_actions.offset(action_index as isize) as *const TSParseActionEntry;
        (*result).action_count = (*entry).c2rust_unnamed.count as uint32_t;
        (*result).is_reusable = (*entry).c2rust_unnamed.reusable();
        (*result).actions = entry.offset(1 as libc::c_int as isize) as *const TSParseAction
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_metadata(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> TSSymbolMetadata {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int {
        return {
            let mut init = TSSymbolMetadata {
                visible_named: [0; 1],
            };
            init.set_visible(1 as libc::c_int != 0);
            init.set_named(1 as libc::c_int != 0);
            init
        };
    } else if symbol as libc::c_int
        == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        return {
            let mut init = TSSymbolMetadata {
                visible_named: [0; 1],
            };
            init.set_visible(0 as libc::c_int != 0);
            init.set_named(0 as libc::c_int != 0);
            init
        };
    } else {
        return *(*self_0).symbol_metadata.offset(symbol as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_public_symbol(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> TSSymbol {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int {
        return symbol;
    }
    if (*self_0).version >= 11 as libc::c_int as libc::c_uint {
        return *(*self_0).public_symbol_map.offset(symbol as isize);
    } else {
        return symbol;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_name(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> *const libc::c_char {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int {
        return b"ERROR\x00" as *const u8 as *const libc::c_char;
    } else if symbol as libc::c_int
        == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        return b"_ERROR\x00" as *const u8 as *const libc::c_char;
    } else if (symbol as libc::c_uint) < ts_language_symbol_count(self_0) {
        return *(*self_0).symbol_names.offset(symbol as isize);
    } else {
        return 0 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_for_name(
    mut self_0: *const TSLanguage,
    mut string: *const libc::c_char,
    mut length: uint32_t,
    mut is_named: bool,
) -> TSSymbol {
    if strncmp(
        string,
        b"ERROR\x00" as *const u8 as *const libc::c_char,
        length as usize,
    ) == 0
    {
        return -(1 as libc::c_int) as TSSymbol;
    }
    let mut count: uint32_t = ts_language_symbol_count(self_0);
    let mut i: TSSymbol = 0 as libc::c_int as TSSymbol;
    while (i as libc::c_uint) < count {
        let mut metadata: TSSymbolMetadata = ts_language_symbol_metadata(self_0, i);
        if !(!metadata.visible() || metadata.named() as libc::c_int != is_named as libc::c_int) {
            let mut symbol_name: *const libc::c_char = *(*self_0).symbol_names.offset(i as isize);
            if strncmp(symbol_name, string, length as usize) == 0
                && *symbol_name.offset(length as isize) == 0
            {
                if (*self_0).version >= 11 as libc::c_int as libc::c_uint {
                    return *(*self_0).public_symbol_map.offset(i as isize);
                } else {
                    return i;
                }
            }
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as TSSymbol;
}
/* *
 * Check whether the given node type id belongs to named nodes, anonymous nodes,
 * or a hidden nodes.
 *
 * See also `ts_node_is_named`. Hidden nodes are never returned from the API.
 */
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_type(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> TSSymbolType {
    let mut metadata: TSSymbolMetadata = ts_language_symbol_metadata(self_0, symbol);
    if metadata.named() {
        return TSSymbolTypeRegular;
    } else if metadata.visible() {
        return TSSymbolTypeAnonymous;
    } else {
        return TSSymbolTypeAuxiliary;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_name_for_id(
    mut self_0: *const TSLanguage,
    mut id: TSFieldId,
) -> *const libc::c_char {
    let mut count: uint32_t = ts_language_field_count(self_0);
    if count != 0 && id as libc::c_uint <= count {
        return *(*self_0).field_names.offset(id as isize);
    } else {
        return 0 as *const libc::c_char;
    };
}

#[no_mangle]
pub unsafe extern "C" fn ts_language_field_id_for_name(
    mut self_0: *const TSLanguage,
    mut name: *const libc::c_char,
    mut name_length: uint32_t,
) -> TSFieldId {
    let mut count: uint32_t = ts_language_field_count(self_0);
    let mut i: TSSymbol = 1 as libc::c_int as TSSymbol;
    while (i as libc::c_uint) < count.wrapping_add(1 as libc::c_int as libc::c_uint) {
        match strncmp(
            name,
            *(*self_0).field_names.offset(i as isize),
            name_length as usize,
        ) {
            0 => {
                if *(*(*self_0).field_names.offset(i as isize)).offset(name_length as isize)
                    as libc::c_int
                    == 0 as libc::c_int
                {
                    return i;
                }
            }
            -1 => return 0 as libc::c_int as TSFieldId,
            _ => {}
        }
        i = i.wrapping_add(1)
    }
    return 0 as libc::c_int as TSFieldId;
}
