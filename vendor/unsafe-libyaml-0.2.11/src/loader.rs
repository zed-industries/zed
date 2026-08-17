use crate::api::{yaml_free, yaml_malloc, yaml_stack_extend, yaml_strdup};
use crate::externs::{memset, strcmp};
use crate::success::{Success, FAIL, OK};
use crate::yaml::yaml_char_t;
use crate::{
    libc, yaml_alias_data_t, yaml_document_delete, yaml_document_t, yaml_event_t, yaml_mark_t,
    yaml_node_item_t, yaml_node_pair_t, yaml_node_t, yaml_parser_parse, yaml_parser_t, PointerExt,
    YAML_ALIAS_EVENT, YAML_COMPOSER_ERROR, YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_START_EVENT,
    YAML_MAPPING_END_EVENT, YAML_MAPPING_NODE, YAML_MAPPING_START_EVENT, YAML_MEMORY_ERROR,
    YAML_SCALAR_EVENT, YAML_SCALAR_NODE, YAML_SEQUENCE_END_EVENT, YAML_SEQUENCE_NODE,
    YAML_SEQUENCE_START_EVENT, YAML_STREAM_END_EVENT, YAML_STREAM_START_EVENT,
};
use core::mem::{size_of, MaybeUninit};
use core::ptr::{self, addr_of_mut};

#[repr(C)]
struct loader_ctx {
    start: *mut libc::c_int,
    end: *mut libc::c_int,
    top: *mut libc::c_int,
}

/// Parse the input stream and produce the next YAML document.
///
/// Call this function subsequently to produce a sequence of documents
/// constituting the input stream.
///
/// If the produced document has no root node, it means that the document end
/// has been reached.
///
/// An application is responsible for freeing any data associated with the
/// produced document object using the yaml_document_delete() function.
///
/// An application must not alternate the calls of yaml_parser_load() with the
/// calls of yaml_parser_scan() or yaml_parser_parse(). Doing this will break
/// the parser.
pub unsafe fn yaml_parser_load(
    parser: *mut yaml_parser_t,
    document: *mut yaml_document_t,
) -> Success {
    let current_block: u64;
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    __assert!(!parser.is_null());
    __assert!(!document.is_null());
    memset(
        document as *mut libc::c_void,
        0,
        size_of::<yaml_document_t>() as libc::c_ulong,
    );
    STACK_INIT!((*document).nodes, yaml_node_t);
    if !(*parser).stream_start_produced {
        if yaml_parser_parse(parser, event).fail {
            current_block = 6234624449317607669;
        } else {
            __assert!((*event).type_ == YAML_STREAM_START_EVENT);
            current_block = 7815301370352969686;
        }
    } else {
        current_block = 7815301370352969686;
    }
    if current_block != 6234624449317607669 {
        if (*parser).stream_end_produced {
            return OK;
        }
        if yaml_parser_parse(parser, event).ok {
            if (*event).type_ == YAML_STREAM_END_EVENT {
                return OK;
            }
            STACK_INIT!((*parser).aliases, yaml_alias_data_t);
            let fresh6 = addr_of_mut!((*parser).document);
            *fresh6 = document;
            if yaml_parser_load_document(parser, event).ok {
                yaml_parser_delete_aliases(parser);
                let fresh7 = addr_of_mut!((*parser).document);
                *fresh7 = ptr::null_mut::<yaml_document_t>();
                return OK;
            }
        }
    }
    yaml_parser_delete_aliases(parser);
    yaml_document_delete(document);
    let fresh8 = addr_of_mut!((*parser).document);
    *fresh8 = ptr::null_mut::<yaml_document_t>();
    FAIL
}

unsafe fn yaml_parser_set_composer_error(
    parser: *mut yaml_parser_t,
    problem: *const libc::c_char,
    problem_mark: yaml_mark_t,
) -> Success {
    (*parser).error = YAML_COMPOSER_ERROR;
    let fresh9 = addr_of_mut!((*parser).problem);
    *fresh9 = problem;
    (*parser).problem_mark = problem_mark;
    FAIL
}

unsafe fn yaml_parser_set_composer_error_context(
    parser: *mut yaml_parser_t,
    context: *const libc::c_char,
    context_mark: yaml_mark_t,
    problem: *const libc::c_char,
    problem_mark: yaml_mark_t,
) -> Success {
    (*parser).error = YAML_COMPOSER_ERROR;
    let fresh10 = addr_of_mut!((*parser).context);
    *fresh10 = context;
    (*parser).context_mark = context_mark;
    let fresh11 = addr_of_mut!((*parser).problem);
    *fresh11 = problem;
    (*parser).problem_mark = problem_mark;
    FAIL
}

unsafe fn yaml_parser_delete_aliases(parser: *mut yaml_parser_t) {
    while !STACK_EMPTY!((*parser).aliases) {
        yaml_free(POP!((*parser).aliases).anchor as *mut libc::c_void);
    }
    STACK_DEL!((*parser).aliases);
}

unsafe fn yaml_parser_load_document(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> Success {
    let mut ctx = loader_ctx {
        start: ptr::null_mut::<libc::c_int>(),
        end: ptr::null_mut::<libc::c_int>(),
        top: ptr::null_mut::<libc::c_int>(),
    };
    __assert!((*event).type_ == YAML_DOCUMENT_START_EVENT);
    let fresh16 = addr_of_mut!((*(*parser).document).version_directive);
    *fresh16 = (*event).data.document_start.version_directive;
    let fresh17 = addr_of_mut!((*(*parser).document).tag_directives.start);
    *fresh17 = (*event).data.document_start.tag_directives.start;
    let fresh18 = addr_of_mut!((*(*parser).document).tag_directives.end);
    *fresh18 = (*event).data.document_start.tag_directives.end;
    (*(*parser).document).start_implicit = (*event).data.document_start.implicit;
    (*(*parser).document).start_mark = (*event).start_mark;
    STACK_INIT!(ctx, libc::c_int);
    if yaml_parser_load_nodes(parser, addr_of_mut!(ctx)).fail {
        STACK_DEL!(ctx);
        return FAIL;
    }
    STACK_DEL!(ctx);
    OK
}

unsafe fn yaml_parser_load_nodes(parser: *mut yaml_parser_t, ctx: *mut loader_ctx) -> Success {
    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    loop {
        if yaml_parser_parse(parser, event).fail {
            return FAIL;
        }
        match (*event).type_ {
            YAML_ALIAS_EVENT => {
                if yaml_parser_load_alias(parser, event, ctx).fail {
                    return FAIL;
                }
            }
            YAML_SCALAR_EVENT => {
                if yaml_parser_load_scalar(parser, event, ctx).fail {
                    return FAIL;
                }
            }
            YAML_SEQUENCE_START_EVENT => {
                if yaml_parser_load_sequence(parser, event, ctx).fail {
                    return FAIL;
                }
            }
            YAML_SEQUENCE_END_EVENT => {
                if yaml_parser_load_sequence_end(parser, event, ctx).fail {
                    return FAIL;
                }
            }
            YAML_MAPPING_START_EVENT => {
                if yaml_parser_load_mapping(parser, event, ctx).fail {
                    return FAIL;
                }
            }
            YAML_MAPPING_END_EVENT => {
                if yaml_parser_load_mapping_end(parser, event, ctx).fail {
                    return FAIL;
                }
            }
            YAML_DOCUMENT_END_EVENT => {}
            _ => {
                __assert!(false);
            }
        }
        if (*event).type_ == YAML_DOCUMENT_END_EVENT {
            break;
        }
    }
    (*(*parser).document).end_implicit = (*event).data.document_end.implicit;
    (*(*parser).document).end_mark = (*event).end_mark;
    OK
}

unsafe fn yaml_parser_register_anchor(
    parser: *mut yaml_parser_t,
    index: libc::c_int,
    anchor: *mut yaml_char_t,
) -> Success {
    let mut data = MaybeUninit::<yaml_alias_data_t>::uninit();
    let data = data.as_mut_ptr();
    let mut alias_data: *mut yaml_alias_data_t;
    if anchor.is_null() {
        return OK;
    }
    (*data).anchor = anchor;
    (*data).index = index;
    (*data).mark = (*(*(*parser).document)
        .nodes
        .start
        .wrapping_offset((index - 1) as isize))
    .start_mark;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp(
            (*alias_data).anchor as *mut libc::c_char,
            anchor as *mut libc::c_char,
        ) == 0
        {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_set_composer_error_context(
                parser,
                b"found duplicate anchor; first occurrence\0" as *const u8 as *const libc::c_char,
                (*alias_data).mark,
                b"second occurrence\0" as *const u8 as *const libc::c_char,
                (*data).mark,
            );
        }
        alias_data = alias_data.wrapping_offset(1);
    }
    PUSH!((*parser).aliases, *data);
    OK
}

unsafe fn yaml_parser_load_node_add(
    parser: *mut yaml_parser_t,
    ctx: *mut loader_ctx,
    index: libc::c_int,
) -> Success {
    if STACK_EMPTY!(*ctx) {
        return OK;
    }
    let parent_index: libc::c_int = *(*ctx).top.wrapping_offset(-1_isize);
    let parent: *mut yaml_node_t = addr_of_mut!(
        *((*(*parser).document).nodes.start).wrapping_offset((parent_index - 1) as isize)
    );
    let current_block_17: u64;
    match (*parent).type_ {
        YAML_SEQUENCE_NODE => {
            if STACK_LIMIT!(parser, (*parent).data.sequence.items).fail {
                return FAIL;
            }
            PUSH!((*parent).data.sequence.items, index);
        }
        YAML_MAPPING_NODE => {
            let mut pair = MaybeUninit::<yaml_node_pair_t>::uninit();
            let pair = pair.as_mut_ptr();
            if !STACK_EMPTY!((*parent).data.mapping.pairs) {
                let p: *mut yaml_node_pair_t =
                    (*parent).data.mapping.pairs.top.wrapping_offset(-1_isize);
                if (*p).key != 0 && (*p).value == 0 {
                    (*p).value = index;
                    current_block_17 = 11307063007268554308;
                } else {
                    current_block_17 = 17407779659766490442;
                }
            } else {
                current_block_17 = 17407779659766490442;
            }
            match current_block_17 {
                11307063007268554308 => {}
                _ => {
                    (*pair).key = index;
                    (*pair).value = 0;
                    if STACK_LIMIT!(parser, (*parent).data.mapping.pairs).fail {
                        return FAIL;
                    }
                    PUSH!((*parent).data.mapping.pairs, *pair);
                }
            }
        }
        _ => {
            __assert!(false);
        }
    }
    OK
}

unsafe fn yaml_parser_load_alias(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> Success {
    let anchor: *mut yaml_char_t = (*event).data.alias.anchor;
    let mut alias_data: *mut yaml_alias_data_t;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp(
            (*alias_data).anchor as *mut libc::c_char,
            anchor as *mut libc::c_char,
        ) == 0
        {
            yaml_free(anchor as *mut libc::c_void);
            return yaml_parser_load_node_add(parser, ctx, (*alias_data).index);
        }
        alias_data = alias_data.wrapping_offset(1);
    }
    yaml_free(anchor as *mut libc::c_void);
    yaml_parser_set_composer_error(
        parser,
        b"found undefined alias\0" as *const u8 as *const libc::c_char,
        (*event).start_mark,
    )
}

unsafe fn yaml_parser_load_scalar(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> Success {
    let current_block: u64;
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    let index: libc::c_int;
    let mut tag: *mut yaml_char_t = (*event).data.scalar.tag;
    if STACK_LIMIT!(parser, (*(*parser).document).nodes).ok {
        if tag.is_null()
            || strcmp(
                tag as *mut libc::c_char,
                b"!\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:str\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 10579931339944277179;
            } else {
                current_block = 11006700562992250127;
            }
        } else {
            current_block = 11006700562992250127;
        }
        if current_block != 10579931339944277179 {
            memset(
                node as *mut libc::c_void,
                0,
                size_of::<yaml_node_t>() as libc::c_ulong,
            );
            (*node).type_ = YAML_SCALAR_NODE;
            (*node).tag = tag;
            (*node).start_mark = (*event).start_mark;
            (*node).end_mark = (*event).end_mark;
            (*node).data.scalar.value = (*event).data.scalar.value;
            (*node).data.scalar.length = (*event).data.scalar.length;
            (*node).data.scalar.style = (*event).data.scalar.style;
            PUSH!((*(*parser).document).nodes, *node);
            index = (*(*parser).document)
                .nodes
                .top
                .c_offset_from((*(*parser).document).nodes.start)
                as libc::c_int;
            if yaml_parser_register_anchor(parser, index, (*event).data.scalar.anchor).fail {
                return FAIL;
            }
            return yaml_parser_load_node_add(parser, ctx, index);
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.scalar.anchor as *mut libc::c_void);
    yaml_free((*event).data.scalar.value as *mut libc::c_void);
    FAIL
}

unsafe fn yaml_parser_load_sequence(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> Success {
    let current_block: u64;
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    struct Items {
        start: *mut yaml_node_item_t,
        end: *mut yaml_node_item_t,
        top: *mut yaml_node_item_t,
    }
    let mut items = Items {
        start: ptr::null_mut::<yaml_node_item_t>(),
        end: ptr::null_mut::<yaml_node_item_t>(),
        top: ptr::null_mut::<yaml_node_item_t>(),
    };
    let index: libc::c_int;
    let mut tag: *mut yaml_char_t = (*event).data.sequence_start.tag;
    if STACK_LIMIT!(parser, (*(*parser).document).nodes).ok {
        if tag.is_null()
            || strcmp(
                tag as *mut libc::c_char,
                b"!\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:seq\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 13474536459355229096;
            } else {
                current_block = 6937071982253665452;
            }
        } else {
            current_block = 6937071982253665452;
        }
        if current_block != 13474536459355229096 {
            STACK_INIT!(items, yaml_node_item_t);
            memset(
                node as *mut libc::c_void,
                0,
                size_of::<yaml_node_t>() as libc::c_ulong,
            );
            (*node).type_ = YAML_SEQUENCE_NODE;
            (*node).tag = tag;
            (*node).start_mark = (*event).start_mark;
            (*node).end_mark = (*event).end_mark;
            (*node).data.sequence.items.start = items.start;
            (*node).data.sequence.items.end = items.end;
            (*node).data.sequence.items.top = items.start;
            (*node).data.sequence.style = (*event).data.sequence_start.style;
            PUSH!((*(*parser).document).nodes, *node);
            index = (*(*parser).document)
                .nodes
                .top
                .c_offset_from((*(*parser).document).nodes.start)
                as libc::c_int;
            if yaml_parser_register_anchor(parser, index, (*event).data.sequence_start.anchor).fail
            {
                return FAIL;
            }
            if yaml_parser_load_node_add(parser, ctx, index).fail {
                return FAIL;
            }
            if STACK_LIMIT!(parser, *ctx).fail {
                return FAIL;
            }
            PUSH!(*ctx, index);
            return OK;
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.sequence_start.anchor as *mut libc::c_void);
    FAIL
}

unsafe fn yaml_parser_load_sequence_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> Success {
    __assert!(((*ctx).top).c_offset_from((*ctx).start) as libc::c_long > 0_i64);
    let index: libc::c_int = *(*ctx).top.wrapping_offset(-1_isize);
    __assert!(
        (*((*(*parser).document).nodes.start).wrapping_offset((index - 1) as isize)).type_
            == YAML_SEQUENCE_NODE
    );
    (*(*(*parser).document)
        .nodes
        .start
        .wrapping_offset((index - 1) as isize))
    .end_mark = (*event).end_mark;
    let _ = POP!(*ctx);
    OK
}

unsafe fn yaml_parser_load_mapping(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> Success {
    let current_block: u64;
    let mut node = MaybeUninit::<yaml_node_t>::uninit();
    let node = node.as_mut_ptr();
    struct Pairs {
        start: *mut yaml_node_pair_t,
        end: *mut yaml_node_pair_t,
        top: *mut yaml_node_pair_t,
    }
    let mut pairs = Pairs {
        start: ptr::null_mut::<yaml_node_pair_t>(),
        end: ptr::null_mut::<yaml_node_pair_t>(),
        top: ptr::null_mut::<yaml_node_pair_t>(),
    };
    let index: libc::c_int;
    let mut tag: *mut yaml_char_t = (*event).data.mapping_start.tag;
    if STACK_LIMIT!(parser, (*(*parser).document).nodes).ok {
        if tag.is_null()
            || strcmp(
                tag as *mut libc::c_char,
                b"!\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            yaml_free(tag as *mut libc::c_void);
            tag = yaml_strdup(
                b"tag:yaml.org,2002:map\0" as *const u8 as *const libc::c_char as *mut yaml_char_t,
            );
            if tag.is_null() {
                current_block = 13635467803606088781;
            } else {
                current_block = 6937071982253665452;
            }
        } else {
            current_block = 6937071982253665452;
        }
        if current_block != 13635467803606088781 {
            STACK_INIT!(pairs, yaml_node_pair_t);
            memset(
                node as *mut libc::c_void,
                0,
                size_of::<yaml_node_t>() as libc::c_ulong,
            );
            (*node).type_ = YAML_MAPPING_NODE;
            (*node).tag = tag;
            (*node).start_mark = (*event).start_mark;
            (*node).end_mark = (*event).end_mark;
            (*node).data.mapping.pairs.start = pairs.start;
            (*node).data.mapping.pairs.end = pairs.end;
            (*node).data.mapping.pairs.top = pairs.start;
            (*node).data.mapping.style = (*event).data.mapping_start.style;
            PUSH!((*(*parser).document).nodes, *node);
            index = (*(*parser).document)
                .nodes
                .top
                .c_offset_from((*(*parser).document).nodes.start)
                as libc::c_int;
            if yaml_parser_register_anchor(parser, index, (*event).data.mapping_start.anchor).fail {
                return FAIL;
            }
            if yaml_parser_load_node_add(parser, ctx, index).fail {
                return FAIL;
            }
            if STACK_LIMIT!(parser, *ctx).fail {
                return FAIL;
            }
            PUSH!(*ctx, index);
            return OK;
        }
    }
    yaml_free(tag as *mut libc::c_void);
    yaml_free((*event).data.mapping_start.anchor as *mut libc::c_void);
    FAIL
}

unsafe fn yaml_parser_load_mapping_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> Success {
    __assert!(((*ctx).top).c_offset_from((*ctx).start) as libc::c_long > 0_i64);
    let index: libc::c_int = *(*ctx).top.wrapping_offset(-1_isize);
    __assert!(
        (*((*(*parser).document).nodes.start).wrapping_offset((index - 1) as isize)).type_
            == YAML_MAPPING_NODE
    );
    (*(*(*parser).document)
        .nodes
        .start
        .wrapping_offset((index - 1) as isize))
    .end_mark = (*event).end_mark;
    let _ = POP!(*ctx);
    OK
}
