// This file was generated by `cargo dev update_lints`.
// Use that command to update this file and do not edit by hand.
// Manual edits will be overwritten.

store.register_group(true, "clippy::pedantic", Some("clippy_pedantic"), vec![
    LintId::of(attrs::INLINE_ALWAYS),
    LintId::of(await_holding_invalid::AWAIT_HOLDING_LOCK),
    LintId::of(await_holding_invalid::AWAIT_HOLDING_REFCELL_REF),
    LintId::of(bit_mask::VERBOSE_BIT_MASK),
    LintId::of(bytecount::NAIVE_BYTECOUNT),
    LintId::of(case_sensitive_file_extension_comparisons::CASE_SENSITIVE_FILE_EXTENSION_COMPARISONS),
    LintId::of(casts::CAST_LOSSLESS),
    LintId::of(casts::CAST_POSSIBLE_TRUNCATION),
    LintId::of(casts::CAST_POSSIBLE_WRAP),
    LintId::of(casts::CAST_PRECISION_LOSS),
    LintId::of(casts::CAST_PTR_ALIGNMENT),
    LintId::of(casts::CAST_SIGN_LOSS),
    LintId::of(casts::PTR_AS_PTR),
    LintId::of(checked_conversions::CHECKED_CONVERSIONS),
    LintId::of(copies::SAME_FUNCTIONS_IN_IF_CONDITION),
    LintId::of(copy_iterator::COPY_ITERATOR),
    LintId::of(default::DEFAULT_TRAIT_ACCESS),
    LintId::of(dereference::EXPLICIT_DEREF_METHODS),
    LintId::of(derive::EXPL_IMPL_CLONE_ON_COPY),
    LintId::of(derive::UNSAFE_DERIVE_DESERIALIZE),
    LintId::of(doc::DOC_MARKDOWN),
    LintId::of(doc::MISSING_ERRORS_DOC),
    LintId::of(doc::MISSING_PANICS_DOC),
    LintId::of(empty_enum::EMPTY_ENUM),
    LintId::of(enum_variants::MODULE_NAME_REPETITIONS),
    LintId::of(eta_reduction::REDUNDANT_CLOSURE_FOR_METHOD_CALLS),
    LintId::of(excessive_bools::FN_PARAMS_EXCESSIVE_BOOLS),
    LintId::of(excessive_bools::STRUCT_EXCESSIVE_BOOLS),
    LintId::of(functions::MUST_USE_CANDIDATE),
    LintId::of(functions::TOO_MANY_LINES),
    LintId::of(if_not_else::IF_NOT_ELSE),
    LintId::of(implicit_hasher::IMPLICIT_HASHER),
    LintId::of(implicit_saturating_sub::IMPLICIT_SATURATING_SUB),
    LintId::of(inconsistent_struct_constructor::INCONSISTENT_STRUCT_CONSTRUCTOR),
    LintId::of(infinite_iter::MAYBE_INFINITE_ITER),
    LintId::of(invalid_upcast_comparisons::INVALID_UPCAST_COMPARISONS),
    LintId::of(items_after_statements::ITEMS_AFTER_STATEMENTS),
    LintId::of(iter_not_returning_iterator::ITER_NOT_RETURNING_ITERATOR),
    LintId::of(large_stack_arrays::LARGE_STACK_ARRAYS),
    LintId::of(let_underscore::LET_UNDERSCORE_DROP),
    LintId::of(literal_representation::LARGE_DIGIT_GROUPS),
    LintId::of(literal_representation::UNREADABLE_LITERAL),
    LintId::of(loops::EXPLICIT_INTO_ITER_LOOP),
    LintId::of(loops::EXPLICIT_ITER_LOOP),
    LintId::of(macro_use::MACRO_USE_IMPORTS),
    LintId::of(manual_assert::MANUAL_ASSERT),
    LintId::of(manual_ok_or::MANUAL_OK_OR),
    LintId::of(match_on_vec_items::MATCH_ON_VEC_ITEMS),
    LintId::of(matches::MATCH_BOOL),
    LintId::of(matches::MATCH_SAME_ARMS),
    LintId::of(matches::MATCH_WILDCARD_FOR_SINGLE_VARIANTS),
    LintId::of(matches::MATCH_WILD_ERR_ARM),
    LintId::of(matches::SINGLE_MATCH_ELSE),
    LintId::of(methods::CLONED_INSTEAD_OF_COPIED),
    LintId::of(methods::FILTER_MAP_NEXT),
    LintId::of(methods::FLAT_MAP_OPTION),
    LintId::of(methods::FROM_ITER_INSTEAD_OF_COLLECT),
    LintId::of(methods::IMPLICIT_CLONE),
    LintId::of(methods::INEFFICIENT_TO_STRING),
    LintId::of(methods::MAP_FLATTEN),
    LintId::of(methods::MAP_UNWRAP_OR),
    LintId::of(misc::FLOAT_CMP),
    LintId::of(misc::USED_UNDERSCORE_BINDING),
    LintId::of(misc_early::UNSEPARATED_LITERAL_SUFFIX),
    LintId::of(mut_mut::MUT_MUT),
    LintId::of(needless_bitwise_bool::NEEDLESS_BITWISE_BOOL),
    LintId::of(needless_borrow::REF_BINDING_TO_REFERENCE),
    LintId::of(needless_continue::NEEDLESS_CONTINUE),
    LintId::of(needless_for_each::NEEDLESS_FOR_EACH),
    LintId::of(needless_pass_by_value::NEEDLESS_PASS_BY_VALUE),
    LintId::of(no_effect::NO_EFFECT_UNDERSCORE_BINDING),
    LintId::of(non_expressive_names::MANY_SINGLE_CHAR_NAMES),
    LintId::of(non_expressive_names::SIMILAR_NAMES),
    LintId::of(pass_by_ref_or_value::LARGE_TYPES_PASSED_BY_VALUE),
    LintId::of(pass_by_ref_or_value::TRIVIALLY_COPY_PASS_BY_REF),
    LintId::of(ranges::RANGE_MINUS_ONE),
    LintId::of(ranges::RANGE_PLUS_ONE),
    LintId::of(redundant_else::REDUNDANT_ELSE),
    LintId::of(ref_option_ref::REF_OPTION_REF),
    LintId::of(semicolon_if_nothing_returned::SEMICOLON_IF_NOTHING_RETURNED),
    LintId::of(strings::STRING_ADD_ASSIGN),
    LintId::of(trait_bounds::TRAIT_DUPLICATION_IN_BOUNDS),
    LintId::of(trait_bounds::TYPE_REPETITION_IN_BOUNDS),
    LintId::of(transmute::TRANSMUTE_PTR_TO_PTR),
    LintId::of(types::LINKEDLIST),
    LintId::of(types::OPTION_OPTION),
    LintId::of(unicode::UNICODE_NOT_NFC),
    LintId::of(unit_types::LET_UNIT_VALUE),
    LintId::of(unnecessary_wraps::UNNECESSARY_WRAPS),
    LintId::of(unnested_or_patterns::UNNESTED_OR_PATTERNS),
    LintId::of(unused_async::UNUSED_ASYNC),
    LintId::of(unused_self::UNUSED_SELF),
    LintId::of(wildcard_imports::ENUM_GLOB_USE),
    LintId::of(wildcard_imports::WILDCARD_IMPORTS),
    LintId::of(zero_sized_map_values::ZERO_SIZED_MAP_VALUES),
])
