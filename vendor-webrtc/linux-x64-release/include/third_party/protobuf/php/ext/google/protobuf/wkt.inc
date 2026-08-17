// This file is generated from the .proto files for the well-known
// types. Do not edit!

ZEND_BEGIN_ARG_INFO_EX(arginfo_lookup, 0, 0, 1)
  ZEND_ARG_INFO(0, key)
ZEND_END_ARG_INFO()

static void google_protobuf_any_proto_AddDescriptor();
static void google_protobuf_api_proto_AddDescriptor();
static void google_protobuf_duration_proto_AddDescriptor();
static void google_protobuf_empty_proto_AddDescriptor();
static void google_protobuf_field_mask_proto_AddDescriptor();
static void google_protobuf_source_context_proto_AddDescriptor();
static void google_protobuf_struct_proto_AddDescriptor();
static void google_protobuf_timestamp_proto_AddDescriptor();
static void google_protobuf_type_proto_AddDescriptor();
static void google_protobuf_wrappers_proto_AddDescriptor();
/* google/protobuf/any.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_Any_ce;

const char google_protobuf_any_proto_descriptor [212] = {
'\n', '\031', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 'a', 'n', 'y', '.', 'p', 'r', 'o', 
't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '\"', '&', '\n', '\003', 'A', 'n', 
'y', '\022', '\020', '\n', '\010', 't', 'y', 'p', 'e', '_', 'u', 'r', 'l', '\030', '\001', ' ', '\001', '(', '\t', '\022', '\r', '\n', '\005', 'v', 'a', 
'l', 'u', 'e', '\030', '\002', ' ', '\001', '(', '\014', 'B', 'v', '\n', '\023', 'c', 'o', 'm', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 
'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', '\010', 'A', 'n', 'y', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', ',', 'g', 'o', 'o', 'g', 
'l', 'e', '.', 'g', 'o', 'l', 'a', 'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 
'p', 'e', 's', '/', 'k', 'n', 'o', 'w', 'n', '/', 'a', 'n', 'y', 'p', 'b', '\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', '\036', 'G', 
'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 'n', 'T', 
'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_any_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/any.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/any.proto", google_protobuf_any_proto_descriptor,
                               sizeof(google_protobuf_any_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_Any, initOnce) {
  google_protobuf_any_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_Any_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_Any, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_Any_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\Any",
                   GPBMetadata_Google_Protobuf_Any_methods);

  GPBMetadata_Google_Protobuf_Any_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_Any */

zend_class_entry* google_protobuf_Any_ce;

static PHP_METHOD(google_protobuf_Any, __construct) {
  google_protobuf_any_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Any, getTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "type_url");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Any, setTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "type_url");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Any, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Any, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

ZEND_BEGIN_ARG_INFO_EX(arginfo_is, 0, 0, 1)
  ZEND_ARG_INFO(0, proto)
ZEND_END_ARG_INFO()

static zend_function_entry google_protobuf_Any_phpmethods[] = {
  PHP_ME(google_protobuf_Any, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Any, getTypeUrl, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Any, setTypeUrl, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Any, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Any, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Any, is, arginfo_is, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Any, pack, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Any, unpack, arginfo_void, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Any_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Any",
                   google_protobuf_Any_phpmethods);

  google_protobuf_Any_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Any_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Any_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Any_ce, message_ce);
}

/* google/protobuf/api.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_Api_ce;

const char google_protobuf_api_proto_descriptor [751] = {
'\n', '\031', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 'a', 'p', 'i', '.', 'p', 'r', 'o', 
't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '\032', '$', 'g', 'o', 'o', 'g', 
'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 's', 'o', 'u', 'r', 'c', 'e', '_', 'c', 'o', 'n', 't', 'e', 'x', 
't', '.', 'p', 'r', 'o', 't', 'o', '\032', '\032', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 
't', 'y', 'p', 'e', '.', 'p', 'r', 'o', 't', 'o', '\"', '\201', '\002', '\n', '\003', 'A', 'p', 'i', '\022', '\014', '\n', '\004', 'n', 'a', 'm', 
'e', '\030', '\001', ' ', '\001', '(', '\t', '\022', '(', '\n', '\007', 'm', 'e', 't', 'h', 'o', 'd', 's', '\030', '\002', ' ', '\003', '(', '\013', '2', 
'\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'M', 'e', 't', 'h', 'o', 'd', '\022', 
'(', '\n', '\007', 'o', 'p', 't', 'i', 'o', 'n', 's', '\030', '\003', ' ', '\003', '(', '\013', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', 
'.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'O', 'p', 't', 'i', 'o', 'n', '\022', '\017', '\n', '\007', 'v', 'e', 'r', 's', 'i', 
'o', 'n', '\030', '\004', ' ', '\001', '(', '\t', '\022', '6', '\n', '\016', 's', 'o', 'u', 'r', 'c', 'e', '_', 'c', 'o', 'n', 't', 'e', 'x', 
't', '\030', '\005', ' ', '\001', '(', '\013', '2', '\036', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 
'.', 'S', 'o', 'u', 'r', 'c', 'e', 'C', 'o', 'n', 't', 'e', 'x', 't', '\022', '&', '\n', '\006', 'm', 'i', 'x', 'i', 'n', 's', '\030', 
'\006', ' ', '\003', '(', '\013', '2', '\026', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'M', 
'i', 'x', 'i', 'n', '\022', '\'', '\n', '\006', 's', 'y', 'n', 't', 'a', 'x', '\030', '\007', ' ', '\001', '(', '\016', '2', '\027', '.', 'g', 'o', 
'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'S', 'y', 'n', 't', 'a', 'x', '\"', '\325', '\001', '\n', '\006', 
'M', 'e', 't', 'h', 'o', 'd', '\022', '\014', '\n', '\004', 'n', 'a', 'm', 'e', '\030', '\001', ' ', '\001', '(', '\t', '\022', '\030', '\n', '\020', 'r', 
'e', 'q', 'u', 'e', 's', 't', '_', 't', 'y', 'p', 'e', '_', 'u', 'r', 'l', '\030', '\002', ' ', '\001', '(', '\t', '\022', '\031', '\n', '\021', 
'r', 'e', 'q', 'u', 'e', 's', 't', '_', 's', 't', 'r', 'e', 'a', 'm', 'i', 'n', 'g', '\030', '\003', ' ', '\001', '(', '\010', '\022', '\031', 
'\n', '\021', 'r', 'e', 's', 'p', 'o', 'n', 's', 'e', '_', 't', 'y', 'p', 'e', '_', 'u', 'r', 'l', '\030', '\004', ' ', '\001', '(', '\t', 
'\022', '\032', '\n', '\022', 'r', 'e', 's', 'p', 'o', 'n', 's', 'e', '_', 's', 't', 'r', 'e', 'a', 'm', 'i', 'n', 'g', '\030', '\005', ' ', 
'\001', '(', '\010', '\022', '(', '\n', '\007', 'o', 'p', 't', 'i', 'o', 'n', 's', '\030', '\006', ' ', '\003', '(', '\013', '2', '\027', '.', 'g', 'o', 
'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'O', 'p', 't', 'i', 'o', 'n', '\022', '\'', '\n', '\006', 's', 
'y', 'n', 't', 'a', 'x', '\030', '\007', ' ', '\001', '(', '\016', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 
'o', 'b', 'u', 'f', '.', 'S', 'y', 'n', 't', 'a', 'x', '\"', '#', '\n', '\005', 'M', 'i', 'x', 'i', 'n', '\022', '\014', '\n', '\004', 'n', 
'a', 'm', 'e', '\030', '\001', ' ', '\001', '(', '\t', '\022', '\014', '\n', '\004', 'r', 'o', 'o', 't', '\030', '\002', ' ', '\001', '(', '\t', 'B', 'v', 
'\n', '\023', 'c', 'o', 'm', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', '\010', 'A', 'p', 
'i', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', ',', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'g', 'o', 'l', 'a', 'n', 'g', '.', 'o', 
'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 's', '/', 'k', 'n', 'o', 'w', 'n', '/', 'a', 
'p', 'i', 'p', 'b', '\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', '\036', 'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 
'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 'n', 'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', 
'3', 
};

static void google_protobuf_api_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/api.proto")) return;
  google_protobuf_source_context_proto_AddDescriptor();
  google_protobuf_type_proto_AddDescriptor();
  DescriptorPool_AddDescriptor("google/protobuf/api.proto", google_protobuf_api_proto_descriptor,
                               sizeof(google_protobuf_api_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_Api, initOnce) {
  google_protobuf_api_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_Api_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_Api, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_Api_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\Api",
                   GPBMetadata_Google_Protobuf_Api_methods);

  GPBMetadata_Google_Protobuf_Api_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_Api */

zend_class_entry* google_protobuf_Api_ce;

static PHP_METHOD(google_protobuf_Api, __construct) {
  google_protobuf_api_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Api, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Api, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Api, getMethods) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "methods");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Api, setMethods) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "methods");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Api, getOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Api, setOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Api, getVersion) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "version");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Api, setVersion) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "version");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Api, getSourceContext) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "source_context");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Api, setSourceContext) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "source_context");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Api, getMixins) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "mixins");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Api, setMixins) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "mixins");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Api, getSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Api, setSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Api_phpmethods[] = {
  PHP_ME(google_protobuf_Api, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, getMethods, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, setMethods, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, getOptions, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, setOptions, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, getVersion, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, setVersion, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, getSourceContext, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, setSourceContext, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, getMixins, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, setMixins, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, getSyntax, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Api, setSyntax, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Api_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Api",
                   google_protobuf_Api_phpmethods);

  google_protobuf_Api_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Api_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Api_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Api_ce, message_ce);
}

/* google_protobuf_Method */

zend_class_entry* google_protobuf_Method_ce;

static PHP_METHOD(google_protobuf_Method, __construct) {
  google_protobuf_api_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Method, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Method, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Method, getRequestTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "request_type_url");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Method, setRequestTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "request_type_url");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Method, getRequestStreaming) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "request_streaming");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Method, setRequestStreaming) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "request_streaming");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Method, getResponseTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "response_type_url");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Method, setResponseTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "response_type_url");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Method, getResponseStreaming) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "response_streaming");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Method, setResponseStreaming) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "response_streaming");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Method, getOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Method, setOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Method, getSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Method, setSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Method_phpmethods[] = {
  PHP_ME(google_protobuf_Method, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, getRequestTypeUrl, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, setRequestTypeUrl, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, getRequestStreaming, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, setRequestStreaming, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, getResponseTypeUrl, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, setResponseTypeUrl, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, getResponseStreaming, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, setResponseStreaming, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, getOptions, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, setOptions, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, getSyntax, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Method, setSyntax, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Method_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Method",
                   google_protobuf_Method_phpmethods);

  google_protobuf_Method_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Method_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Method_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Method_ce, message_ce);
}

/* google_protobuf_Mixin */

zend_class_entry* google_protobuf_Mixin_ce;

static PHP_METHOD(google_protobuf_Mixin, __construct) {
  google_protobuf_api_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Mixin, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Mixin, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Mixin, getRoot) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "root");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Mixin, setRoot) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "root");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Mixin_phpmethods[] = {
  PHP_ME(google_protobuf_Mixin, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Mixin, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Mixin, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Mixin, getRoot, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Mixin, setRoot, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Mixin_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Mixin",
                   google_protobuf_Mixin_phpmethods);

  google_protobuf_Mixin_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Mixin_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Mixin_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Mixin_ce, message_ce);
}

/* google/protobuf/duration.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_Duration_ce;

const char google_protobuf_duration_proto_descriptor [235] = {
'\n', '\036', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 'd', 'u', 'r', 'a', 't', 'i', 'o', 
'n', '.', 'p', 'r', 'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '\"', 
'*', '\n', '\010', 'D', 'u', 'r', 'a', 't', 'i', 'o', 'n', '\022', '\017', '\n', '\007', 's', 'e', 'c', 'o', 'n', 'd', 's', '\030', '\001', ' ', 
'\001', '(', '\003', '\022', '\r', '\n', '\005', 'n', 'a', 'n', 'o', 's', '\030', '\002', ' ', '\001', '(', '\005', 'B', '\203', '\001', '\n', '\023', 'c', 'o', 
'm', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', '\r', 'D', 'u', 'r', 'a', 't', 'i', 
'o', 'n', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', '1', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'g', 'o', 'l', 'a', 'n', 'g', '.', 
'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 's', '/', 'k', 'n', 'o', 'w', 'n', '/', 
'd', 'u', 'r', 'a', 't', 'i', 'o', 'n', 'p', 'b', '\370', '\001', '\001', '\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', '\036', 'G', 'o', 'o', 
'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 'n', 'T', 'y', 'p', 
'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_duration_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/duration.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/duration.proto", google_protobuf_duration_proto_descriptor,
                               sizeof(google_protobuf_duration_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_Duration, initOnce) {
  google_protobuf_duration_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_Duration_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_Duration, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_Duration_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\Duration",
                   GPBMetadata_Google_Protobuf_Duration_methods);

  GPBMetadata_Google_Protobuf_Duration_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_Duration */

zend_class_entry* google_protobuf_Duration_ce;

static PHP_METHOD(google_protobuf_Duration, __construct) {
  google_protobuf_duration_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Duration, getSeconds) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "seconds");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Duration, setSeconds) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "seconds");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Duration, getNanos) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "nanos");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Duration, setNanos) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "nanos");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Duration_phpmethods[] = {
  PHP_ME(google_protobuf_Duration, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Duration, getSeconds, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Duration, setSeconds, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Duration, getNanos, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Duration, setNanos, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Duration_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Duration",
                   google_protobuf_Duration_phpmethods);

  google_protobuf_Duration_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Duration_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Duration_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Duration_ce, message_ce);
}

/* google/protobuf/empty.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_GPBEmpty_ce;

const char google_protobuf_empty_proto_descriptor [190] = {
'\n', '\033', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 'e', 'm', 'p', 't', 'y', '.', 'p', 
'r', 'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '\"', '\007', '\n', '\005', 
'E', 'm', 'p', 't', 'y', 'B', '}', '\n', '\023', 'c', 'o', 'm', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 
'b', 'u', 'f', 'B', '\n', 'E', 'm', 'p', 't', 'y', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', '.', 'g', 'o', 'o', 'g', 'l', 'e', 
'.', 'g', 'o', 'l', 'a', 'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 
's', '/', 'k', 'n', 'o', 'w', 'n', '/', 'e', 'm', 'p', 't', 'y', 'p', 'b', '\370', '\001', '\001', '\242', '\002', '\003', 'G', 'P', 'B', '\252', 
'\002', '\036', 'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 
'w', 'n', 'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_empty_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/empty.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/empty.proto", google_protobuf_empty_proto_descriptor,
                               sizeof(google_protobuf_empty_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_GPBEmpty, initOnce) {
  google_protobuf_empty_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_GPBEmpty_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_GPBEmpty, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_GPBEmpty_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\GPBEmpty",
                   GPBMetadata_Google_Protobuf_GPBEmpty_methods);

  GPBMetadata_Google_Protobuf_GPBEmpty_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_Empty */

zend_class_entry* google_protobuf_Empty_ce;

static PHP_METHOD(google_protobuf_Empty, __construct) {
  google_protobuf_empty_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static zend_function_entry google_protobuf_Empty_phpmethods[] = {
  PHP_ME(google_protobuf_Empty, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Empty_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\GPBEmpty",
                   google_protobuf_Empty_phpmethods);

  google_protobuf_Empty_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Empty_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Empty_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Empty_ce, message_ce);
}

/* google/protobuf/field_mask.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_FieldMask_ce;

const char google_protobuf_field_mask_proto_descriptor [223] = {
'\n', ' ', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 'f', 'i', 'e', 'l', 'd', '_', 'm', 
'a', 's', 'k', '.', 'p', 'r', 'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 
'f', '\"', '\032', '\n', '\t', 'F', 'i', 'e', 'l', 'd', 'M', 'a', 's', 'k', '\022', '\r', '\n', '\005', 'p', 'a', 't', 'h', 's', '\030', '\001', 
' ', '\003', '(', '\t', 'B', '\205', '\001', '\n', '\023', 'c', 'o', 'm', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 
'b', 'u', 'f', 'B', '\016', 'F', 'i', 'e', 'l', 'd', 'M', 'a', 's', 'k', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', '2', 'g', 'o', 
'o', 'g', 'l', 'e', '.', 'g', 'o', 'l', 'a', 'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 
't', 'y', 'p', 'e', 's', '/', 'k', 'n', 'o', 'w', 'n', '/', 'f', 'i', 'e', 'l', 'd', 'm', 'a', 's', 'k', 'p', 'b', '\370', '\001', 
'\001', '\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', '\036', 'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', 
'.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 'n', 'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_field_mask_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/field_mask.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/field_mask.proto", google_protobuf_field_mask_proto_descriptor,
                               sizeof(google_protobuf_field_mask_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_FieldMask, initOnce) {
  google_protobuf_field_mask_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_FieldMask_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_FieldMask, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_FieldMask_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\FieldMask",
                   GPBMetadata_Google_Protobuf_FieldMask_methods);

  GPBMetadata_Google_Protobuf_FieldMask_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_FieldMask */

zend_class_entry* google_protobuf_FieldMask_ce;

static PHP_METHOD(google_protobuf_FieldMask, __construct) {
  google_protobuf_field_mask_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_FieldMask, getPaths) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "paths");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_FieldMask, setPaths) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "paths");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_FieldMask_phpmethods[] = {
  PHP_ME(google_protobuf_FieldMask, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_FieldMask, getPaths, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_FieldMask, setPaths, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_FieldMask_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\FieldMask",
                   google_protobuf_FieldMask_phpmethods);

  google_protobuf_FieldMask_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_FieldMask_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_FieldMask_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_FieldMask_ce, message_ce);
}

/* google/protobuf/source_context.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_SourceContext_ce;

const char google_protobuf_source_context_proto_descriptor [240] = {
'\n', '$', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 's', 'o', 'u', 'r', 'c', 'e', '_', 
'c', 'o', 'n', 't', 'e', 'x', 't', '.', 'p', 'r', 'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 
't', 'o', 'b', 'u', 'f', '\"', '\"', '\n', '\r', 'S', 'o', 'u', 'r', 'c', 'e', 'C', 'o', 'n', 't', 'e', 'x', 't', '\022', '\021', '\n', 
'\t', 'f', 'i', 'l', 'e', '_', 'n', 'a', 'm', 'e', '\030', '\001', ' ', '\001', '(', '\t', 'B', '\212', '\001', '\n', '\023', 'c', 'o', 'm', '.', 
'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', '\022', 'S', 'o', 'u', 'r', 'c', 'e', 'C', 'o', 
'n', 't', 'e', 'x', 't', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', '6', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'g', 'o', 'l', 'a', 
'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 's', '/', 'k', 'n', 'o', 
'w', 'n', '/', 's', 'o', 'u', 'r', 'c', 'e', 'c', 'o', 'n', 't', 'e', 'x', 't', 'p', 'b', '\242', '\002', '\003', 'G', 'P', 'B', '\252', 
'\002', '\036', 'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 
'w', 'n', 'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_source_context_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/source_context.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/source_context.proto", google_protobuf_source_context_proto_descriptor,
                               sizeof(google_protobuf_source_context_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_SourceContext, initOnce) {
  google_protobuf_source_context_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_SourceContext_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_SourceContext, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_SourceContext_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\SourceContext",
                   GPBMetadata_Google_Protobuf_SourceContext_methods);

  GPBMetadata_Google_Protobuf_SourceContext_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_SourceContext */

zend_class_entry* google_protobuf_SourceContext_ce;

static PHP_METHOD(google_protobuf_SourceContext, __construct) {
  google_protobuf_source_context_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_SourceContext, getFileName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "file_name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_SourceContext, setFileName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "file_name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_SourceContext_phpmethods[] = {
  PHP_ME(google_protobuf_SourceContext, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_SourceContext, getFileName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_SourceContext, setFileName, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_SourceContext_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\SourceContext",
                   google_protobuf_SourceContext_phpmethods);

  google_protobuf_SourceContext_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_SourceContext_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_SourceContext_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_SourceContext_ce, message_ce);
}

/* google/protobuf/struct.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_Struct_ce;

const char google_protobuf_struct_proto_descriptor [638] = {
'\n', '\034', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 's', 't', 'r', 'u', 'c', 't', '.', 
'p', 'r', 'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '\"', '\204', '\001', 
'\n', '\006', 'S', 't', 'r', 'u', 'c', 't', '\022', '3', '\n', '\006', 'f', 'i', 'e', 'l', 'd', 's', '\030', '\001', ' ', '\003', '(', '\013', '2', 
'#', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'S', 't', 'r', 'u', 'c', 't', '.', 
'F', 'i', 'e', 'l', 'd', 's', 'E', 'n', 't', 'r', 'y', '\032', 'E', '\n', '\013', 'F', 'i', 'e', 'l', 'd', 's', 'E', 'n', 't', 'r', 
'y', '\022', '\013', '\n', '\003', 'k', 'e', 'y', '\030', '\001', ' ', '\001', '(', '\t', '\022', '%', '\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\002', 
' ', '\001', '(', '\013', '2', '\026', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'V', 'a', 
'l', 'u', 'e', ':', '\002', '8', '\001', '\"', '\352', '\001', '\n', '\005', 'V', 'a', 'l', 'u', 'e', '\022', '0', '\n', '\n', 'n', 'u', 'l', 'l', 
'_', 'v', 'a', 'l', 'u', 'e', '\030', '\001', ' ', '\001', '(', '\016', '2', '\032', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 
't', 'o', 'b', 'u', 'f', '.', 'N', 'u', 'l', 'l', 'V', 'a', 'l', 'u', 'e', 'H', '\000', '\022', '\026', '\n', '\014', 'n', 'u', 'm', 'b', 
'e', 'r', '_', 'v', 'a', 'l', 'u', 'e', '\030', '\002', ' ', '\001', '(', '\001', 'H', '\000', '\022', '\026', '\n', '\014', 's', 't', 'r', 'i', 'n', 
'g', '_', 'v', 'a', 'l', 'u', 'e', '\030', '\003', ' ', '\001', '(', '\t', 'H', '\000', '\022', '\024', '\n', '\n', 'b', 'o', 'o', 'l', '_', 'v', 
'a', 'l', 'u', 'e', '\030', '\004', ' ', '\001', '(', '\010', 'H', '\000', '\022', '/', '\n', '\014', 's', 't', 'r', 'u', 'c', 't', '_', 'v', 'a', 
'l', 'u', 'e', '\030', '\005', ' ', '\001', '(', '\013', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 
'u', 'f', '.', 'S', 't', 'r', 'u', 'c', 't', 'H', '\000', '\022', '0', '\n', '\n', 'l', 'i', 's', 't', '_', 'v', 'a', 'l', 'u', 'e', 
'\030', '\006', ' ', '\001', '(', '\013', '2', '\032', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 
'L', 'i', 's', 't', 'V', 'a', 'l', 'u', 'e', 'H', '\000', 'B', '\006', '\n', '\004', 'k', 'i', 'n', 'd', '\"', '3', '\n', '\t', 'L', 'i', 
's', 't', 'V', 'a', 'l', 'u', 'e', '\022', '&', '\n', '\006', 'v', 'a', 'l', 'u', 'e', 's', '\030', '\001', ' ', '\003', '(', '\013', '2', '\026', 
'.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'V', 'a', 'l', 'u', 'e', '*', '\033', '\n', 
'\t', 'N', 'u', 'l', 'l', 'V', 'a', 'l', 'u', 'e', '\022', '\016', '\n', '\n', 'N', 'U', 'L', 'L', '_', 'V', 'A', 'L', 'U', 'E', '\020', 
'\000', 'B', '\177', '\n', '\023', 'c', 'o', 'm', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', 
'\013', 'S', 't', 'r', 'u', 'c', 't', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', '/', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'g', 'o', 
'l', 'a', 'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 's', '/', 'k', 
'n', 'o', 'w', 'n', '/', 's', 't', 'r', 'u', 'c', 't', 'p', 'b', '\370', '\001', '\001', '\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', '\036', 
'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 'n', 
'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_struct_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/struct.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/struct.proto", google_protobuf_struct_proto_descriptor,
                               sizeof(google_protobuf_struct_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_Struct, initOnce) {
  google_protobuf_struct_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_Struct_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_Struct, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_Struct_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\Struct",
                   GPBMetadata_Google_Protobuf_Struct_methods);

  GPBMetadata_Google_Protobuf_Struct_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_Struct */

zend_class_entry* google_protobuf_Struct_ce;

static PHP_METHOD(google_protobuf_Struct, __construct) {
  google_protobuf_struct_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Struct, getFields) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "fields");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Struct, setFields) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "fields");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Struct_phpmethods[] = {
  PHP_ME(google_protobuf_Struct, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Struct, getFields, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Struct, setFields, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Struct_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Struct",
                   google_protobuf_Struct_phpmethods);

  google_protobuf_Struct_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Struct_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Struct_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Struct_ce, message_ce);
}

/* google_protobuf_Struct_FieldsEntry */

zend_class_entry* google_protobuf_Struct_FieldsEntry_ce;

static PHP_METHOD(google_protobuf_Struct_FieldsEntry, __construct) {
  google_protobuf_struct_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Struct_FieldsEntry, getKey) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "key");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Struct_FieldsEntry, setKey) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "key");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Struct_FieldsEntry, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Struct_FieldsEntry, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Struct_FieldsEntry_phpmethods[] = {
  PHP_ME(google_protobuf_Struct_FieldsEntry, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Struct_FieldsEntry, getKey, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Struct_FieldsEntry, setKey, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Struct_FieldsEntry, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Struct_FieldsEntry, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Struct_FieldsEntry_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Struct\\FieldsEntry",
                   google_protobuf_Struct_FieldsEntry_phpmethods);

  google_protobuf_Struct_FieldsEntry_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Struct_FieldsEntry_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Struct_FieldsEntry_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Struct_FieldsEntry_ce, message_ce);
}

/* google_protobuf_Value */

zend_class_entry* google_protobuf_Value_ce;

static PHP_METHOD(google_protobuf_Value, __construct) {
  google_protobuf_struct_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Value, getNullValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "null_value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Value, setNullValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "null_value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Value, getNumberValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "number_value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Value, setNumberValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "number_value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Value, getStringValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "string_value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Value, setStringValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "string_value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Value, getBoolValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "bool_value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Value, setBoolValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "bool_value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Value, getStructValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "struct_value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Value, setStructValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "struct_value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Value, getListValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "list_value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Value, setListValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "list_value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Value, getKind) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_OneofDef *oneof = upb_MessageDef_FindOneofByName(
      intern->desc->msgdef, "kind");
  const upb_FieldDef *field = 
      upb_Message_WhichOneofByDef(intern->msg, oneof);
  RETURN_STRING(field ? upb_FieldDef_Name(field) : "");
}
static zend_function_entry google_protobuf_Value_phpmethods[] = {
  PHP_ME(google_protobuf_Value, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, getNullValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, setNullValue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, getNumberValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, setNumberValue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, getStringValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, setStringValue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, getBoolValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, setBoolValue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, getStructValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, setStructValue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, getListValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, setListValue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Value, getKind, arginfo_void, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Value_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Value",
                   google_protobuf_Value_phpmethods);

  google_protobuf_Value_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Value_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Value_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Value_ce, message_ce);
}

/* google_protobuf_ListValue */

zend_class_entry* google_protobuf_ListValue_ce;

static PHP_METHOD(google_protobuf_ListValue, __construct) {
  google_protobuf_struct_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_ListValue, getValues) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "values");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_ListValue, setValues) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "values");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_ListValue_phpmethods[] = {
  PHP_ME(google_protobuf_ListValue, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_ListValue, getValues, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_ListValue, setValues, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_ListValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\ListValue",
                   google_protobuf_ListValue_phpmethods);

  google_protobuf_ListValue_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_ListValue_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_ListValue_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_ListValue_ce, message_ce);
}

/* google_protobuf_NullValue */

zend_class_entry* google_protobuf_NullValue_ce;

PHP_METHOD(google_protobuf_NullValue, name) {
  google_protobuf_struct_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.NullValue");
  zend_long value;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &value) ==
      FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev =
      upb_EnumDef_FindValueByNumber(e, value);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\NullValue has no name "
                            "defined for value " ZEND_LONG_FMT ".",
                            value);
    return;
  }
  RETURN_STRING(upb_EnumValueDef_Name(ev));
}

PHP_METHOD(google_protobuf_NullValue, value) {
  google_protobuf_struct_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.NullValue");
  char *name = NULL;
  size_t name_len;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &name,
                            &name_len) == FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev = upb_EnumDef_FindValueByNameWithSize(
      e, name, name_len);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\NullValue has no value "
                            "defined for name %s.",
                            name);
    return;
  }
  RETURN_LONG(upb_EnumValueDef_Number(ev));
}

static zend_function_entry google_protobuf_NullValue_phpmethods[] = {
  PHP_ME(google_protobuf_NullValue, name, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  PHP_ME(google_protobuf_NullValue, value, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void google_protobuf_NullValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\NullValue",
                   google_protobuf_NullValue_phpmethods);

  google_protobuf_NullValue_ce = zend_register_internal_class(&tmp_ce);
  zend_declare_class_constant_long(google_protobuf_NullValue_ce, "NULL_VALUE",
                                   strlen("NULL_VALUE"), 0);
}

/* google/protobuf/timestamp.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_Timestamp_ce;

const char google_protobuf_timestamp_proto_descriptor [239] = {
'\n', '\037', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'i', 'm', 'e', 's', 't', 'a', 
'm', 'p', '.', 'p', 'r', 'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 
'\"', '+', '\n', '\t', 'T', 'i', 'm', 'e', 's', 't', 'a', 'm', 'p', '\022', '\017', '\n', '\007', 's', 'e', 'c', 'o', 'n', 'd', 's', '\030', 
'\001', ' ', '\001', '(', '\003', '\022', '\r', '\n', '\005', 'n', 'a', 'n', 'o', 's', '\030', '\002', ' ', '\001', '(', '\005', 'B', '\205', '\001', '\n', '\023', 
'c', 'o', 'm', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', '\016', 'T', 'i', 'm', 'e', 
's', 't', 'a', 'm', 'p', 'P', 'r', 'o', 't', 'o', 'P', '\001', 'Z', '2', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'g', 'o', 'l', 'a', 
'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 's', '/', 'k', 'n', 'o', 
'w', 'n', '/', 't', 'i', 'm', 'e', 's', 't', 'a', 'm', 'p', 'p', 'b', '\370', '\001', '\001', '\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', 
'\036', 'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 
'n', 'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_timestamp_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/timestamp.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/timestamp.proto", google_protobuf_timestamp_proto_descriptor,
                               sizeof(google_protobuf_timestamp_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_Timestamp, initOnce) {
  google_protobuf_timestamp_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_Timestamp_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_Timestamp, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_Timestamp_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\Timestamp",
                   GPBMetadata_Google_Protobuf_Timestamp_methods);

  GPBMetadata_Google_Protobuf_Timestamp_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_Timestamp */

zend_class_entry* google_protobuf_Timestamp_ce;

static PHP_METHOD(google_protobuf_Timestamp, __construct) {
  google_protobuf_timestamp_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Timestamp, getSeconds) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "seconds");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Timestamp, setSeconds) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "seconds");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Timestamp, getNanos) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "nanos");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Timestamp, setNanos) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "nanos");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

ZEND_BEGIN_ARG_INFO_EX(arginfo_timestamp_fromdatetime, 0, 0, 1)
  ZEND_ARG_INFO(0, datetime)
ZEND_END_ARG_INFO()

static zend_function_entry google_protobuf_Timestamp_phpmethods[] = {
  PHP_ME(google_protobuf_Timestamp, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Timestamp, getSeconds, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Timestamp, setSeconds, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Timestamp, getNanos, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Timestamp, setNanos, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Timestamp, fromDateTime, arginfo_timestamp_fromdatetime, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Timestamp, toDateTime, arginfo_void, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Timestamp_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Timestamp",
                   google_protobuf_Timestamp_phpmethods);

  google_protobuf_Timestamp_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Timestamp_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Timestamp_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Timestamp_ce, message_ce);
}

/* google/protobuf/type.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_Type_ce;

const char google_protobuf_type_proto_descriptor [1647] = {
'\n', '\032', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', '.', 'p', 'r', 
'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '\032', '\031', 'g', 'o', 'o', 
'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 'a', 'n', 'y', '.', 'p', 'r', 'o', 't', 'o', '\032', '$', 'g', 
'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 's', 'o', 'u', 'r', 'c', 'e', '_', 'c', 'o', 'n', 
't', 'e', 'x', 't', '.', 'p', 'r', 'o', 't', 'o', '\"', '\350', '\001', '\n', '\004', 'T', 'y', 'p', 'e', '\022', '\014', '\n', '\004', 'n', 'a', 
'm', 'e', '\030', '\001', ' ', '\001', '(', '\t', '\022', '&', '\n', '\006', 'f', 'i', 'e', 'l', 'd', 's', '\030', '\002', ' ', '\003', '(', '\013', '2', 
'\026', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'F', 'i', 'e', 'l', 'd', '\022', '\016', 
'\n', '\006', 'o', 'n', 'e', 'o', 'f', 's', '\030', '\003', ' ', '\003', '(', '\t', '\022', '(', '\n', '\007', 'o', 'p', 't', 'i', 'o', 'n', 's', 
'\030', '\004', ' ', '\003', '(', '\013', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 
'O', 'p', 't', 'i', 'o', 'n', '\022', '6', '\n', '\016', 's', 'o', 'u', 'r', 'c', 'e', '_', 'c', 'o', 'n', 't', 'e', 'x', 't', '\030', 
'\005', ' ', '\001', '(', '\013', '2', '\036', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'S', 
'o', 'u', 'r', 'c', 'e', 'C', 'o', 'n', 't', 'e', 'x', 't', '\022', '\'', '\n', '\006', 's', 'y', 'n', 't', 'a', 'x', '\030', '\006', ' ', 
'\001', '(', '\016', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'S', 'y', 'n', 
't', 'a', 'x', '\022', '\017', '\n', '\007', 'e', 'd', 'i', 't', 'i', 'o', 'n', '\030', '\007', ' ', '\001', '(', '\t', '\"', '\325', '\005', '\n', '\005', 
'F', 'i', 'e', 'l', 'd', '\022', ')', '\n', '\004', 'k', 'i', 'n', 'd', '\030', '\001', ' ', '\001', '(', '\016', '2', '\033', '.', 'g', 'o', 'o', 
'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'F', 'i', 'e', 'l', 'd', '.', 'K', 'i', 'n', 'd', '\022', '7', 
'\n', '\013', 'c', 'a', 'r', 'd', 'i', 'n', 'a', 'l', 'i', 't', 'y', '\030', '\002', ' ', '\001', '(', '\016', '2', '\"', '.', 'g', 'o', 'o', 
'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'F', 'i', 'e', 'l', 'd', '.', 'C', 'a', 'r', 'd', 'i', 'n', 
'a', 'l', 'i', 't', 'y', '\022', '\016', '\n', '\006', 'n', 'u', 'm', 'b', 'e', 'r', '\030', '\003', ' ', '\001', '(', '\005', '\022', '\014', '\n', '\004', 
'n', 'a', 'm', 'e', '\030', '\004', ' ', '\001', '(', '\t', '\022', '\020', '\n', '\010', 't', 'y', 'p', 'e', '_', 'u', 'r', 'l', '\030', '\006', ' ', 
'\001', '(', '\t', '\022', '\023', '\n', '\013', 'o', 'n', 'e', 'o', 'f', '_', 'i', 'n', 'd', 'e', 'x', '\030', '\007', ' ', '\001', '(', '\005', '\022', 
'\016', '\n', '\006', 'p', 'a', 'c', 'k', 'e', 'd', '\030', '\010', ' ', '\001', '(', '\010', '\022', '(', '\n', '\007', 'o', 'p', 't', 'i', 'o', 'n', 
's', '\030', '\t', ' ', '\003', '(', '\013', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 
'.', 'O', 'p', 't', 'i', 'o', 'n', '\022', '\021', '\n', '\t', 'j', 's', 'o', 'n', '_', 'n', 'a', 'm', 'e', '\030', '\n', ' ', '\001', '(', 
'\t', '\022', '\025', '\n', '\r', 'd', 'e', 'f', 'a', 'u', 'l', 't', '_', 'v', 'a', 'l', 'u', 'e', '\030', '\013', ' ', '\001', '(', '\t', '\"', 
'\310', '\002', '\n', '\004', 'K', 'i', 'n', 'd', '\022', '\020', '\n', '\014', 'T', 'Y', 'P', 'E', '_', 'U', 'N', 'K', 'N', 'O', 'W', 'N', '\020', 
'\000', '\022', '\017', '\n', '\013', 'T', 'Y', 'P', 'E', '_', 'D', 'O', 'U', 'B', 'L', 'E', '\020', '\001', '\022', '\016', '\n', '\n', 'T', 'Y', 'P', 
'E', '_', 'F', 'L', 'O', 'A', 'T', '\020', '\002', '\022', '\016', '\n', '\n', 'T', 'Y', 'P', 'E', '_', 'I', 'N', 'T', '6', '4', '\020', '\003', 
'\022', '\017', '\n', '\013', 'T', 'Y', 'P', 'E', '_', 'U', 'I', 'N', 'T', '6', '4', '\020', '\004', '\022', '\016', '\n', '\n', 'T', 'Y', 'P', 'E', 
'_', 'I', 'N', 'T', '3', '2', '\020', '\005', '\022', '\020', '\n', '\014', 'T', 'Y', 'P', 'E', '_', 'F', 'I', 'X', 'E', 'D', '6', '4', '\020', 
'\006', '\022', '\020', '\n', '\014', 'T', 'Y', 'P', 'E', '_', 'F', 'I', 'X', 'E', 'D', '3', '2', '\020', '\007', '\022', '\r', '\n', '\t', 'T', 'Y', 
'P', 'E', '_', 'B', 'O', 'O', 'L', '\020', '\010', '\022', '\017', '\n', '\013', 'T', 'Y', 'P', 'E', '_', 'S', 'T', 'R', 'I', 'N', 'G', '\020', 
'\t', '\022', '\016', '\n', '\n', 'T', 'Y', 'P', 'E', '_', 'G', 'R', 'O', 'U', 'P', '\020', '\n', '\022', '\020', '\n', '\014', 'T', 'Y', 'P', 'E', 
'_', 'M', 'E', 'S', 'S', 'A', 'G', 'E', '\020', '\013', '\022', '\016', '\n', '\n', 'T', 'Y', 'P', 'E', '_', 'B', 'Y', 'T', 'E', 'S', '\020', 
'\014', '\022', '\017', '\n', '\013', 'T', 'Y', 'P', 'E', '_', 'U', 'I', 'N', 'T', '3', '2', '\020', '\r', '\022', '\r', '\n', '\t', 'T', 'Y', 'P', 
'E', '_', 'E', 'N', 'U', 'M', '\020', '\016', '\022', '\021', '\n', '\r', 'T', 'Y', 'P', 'E', '_', 'S', 'F', 'I', 'X', 'E', 'D', '3', '2', 
'\020', '\017', '\022', '\021', '\n', '\r', 'T', 'Y', 'P', 'E', '_', 'S', 'F', 'I', 'X', 'E', 'D', '6', '4', '\020', '\020', '\022', '\017', '\n', '\013', 
'T', 'Y', 'P', 'E', '_', 'S', 'I', 'N', 'T', '3', '2', '\020', '\021', '\022', '\017', '\n', '\013', 'T', 'Y', 'P', 'E', '_', 'S', 'I', 'N', 
'T', '6', '4', '\020', '\022', '\"', 't', '\n', '\013', 'C', 'a', 'r', 'd', 'i', 'n', 'a', 'l', 'i', 't', 'y', '\022', '\027', '\n', '\023', 'C', 
'A', 'R', 'D', 'I', 'N', 'A', 'L', 'I', 'T', 'Y', '_', 'U', 'N', 'K', 'N', 'O', 'W', 'N', '\020', '\000', '\022', '\030', '\n', '\024', 'C', 
'A', 'R', 'D', 'I', 'N', 'A', 'L', 'I', 'T', 'Y', '_', 'O', 'P', 'T', 'I', 'O', 'N', 'A', 'L', '\020', '\001', '\022', '\030', '\n', '\024', 
'C', 'A', 'R', 'D', 'I', 'N', 'A', 'L', 'I', 'T', 'Y', '_', 'R', 'E', 'Q', 'U', 'I', 'R', 'E', 'D', '\020', '\002', '\022', '\030', '\n', 
'\024', 'C', 'A', 'R', 'D', 'I', 'N', 'A', 'L', 'I', 'T', 'Y', '_', 'R', 'E', 'P', 'E', 'A', 'T', 'E', 'D', '\020', '\003', '\"', '\337', 
'\001', '\n', '\004', 'E', 'n', 'u', 'm', '\022', '\014', '\n', '\004', 'n', 'a', 'm', 'e', '\030', '\001', ' ', '\001', '(', '\t', '\022', '-', '\n', '\t', 
'e', 'n', 'u', 'm', 'v', 'a', 'l', 'u', 'e', '\030', '\002', ' ', '\003', '(', '\013', '2', '\032', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 
'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'E', 'n', 'u', 'm', 'V', 'a', 'l', 'u', 'e', '\022', '(', '\n', '\007', 'o', 'p', 't', 
'i', 'o', 'n', 's', '\030', '\003', ' ', '\003', '(', '\013', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 
'b', 'u', 'f', '.', 'O', 'p', 't', 'i', 'o', 'n', '\022', '6', '\n', '\016', 's', 'o', 'u', 'r', 'c', 'e', '_', 'c', 'o', 'n', 't', 
'e', 'x', 't', '\030', '\004', ' ', '\001', '(', '\013', '2', '\036', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 
'u', 'f', '.', 'S', 'o', 'u', 'r', 'c', 'e', 'C', 'o', 'n', 't', 'e', 'x', 't', '\022', '\'', '\n', '\006', 's', 'y', 'n', 't', 'a', 
'x', '\030', '\005', ' ', '\001', '(', '\016', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 
'.', 'S', 'y', 'n', 't', 'a', 'x', '\022', '\017', '\n', '\007', 'e', 'd', 'i', 't', 'i', 'o', 'n', '\030', '\006', ' ', '\001', '(', '\t', '\"', 
'S', '\n', '\t', 'E', 'n', 'u', 'm', 'V', 'a', 'l', 'u', 'e', '\022', '\014', '\n', '\004', 'n', 'a', 'm', 'e', '\030', '\001', ' ', '\001', '(', 
'\t', '\022', '\016', '\n', '\006', 'n', 'u', 'm', 'b', 'e', 'r', '\030', '\002', ' ', '\001', '(', '\005', '\022', '(', '\n', '\007', 'o', 'p', 't', 'i', 
'o', 'n', 's', '\030', '\003', ' ', '\003', '(', '\013', '2', '\027', '.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 
'u', 'f', '.', 'O', 'p', 't', 'i', 'o', 'n', '\"', ';', '\n', '\006', 'O', 'p', 't', 'i', 'o', 'n', '\022', '\014', '\n', '\004', 'n', 'a', 
'm', 'e', '\030', '\001', ' ', '\001', '(', '\t', '\022', '#', '\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\002', ' ', '\001', '(', '\013', '2', '\024', 
'.', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'A', 'n', 'y', '*', 'C', '\n', '\006', 'S', 
'y', 'n', 't', 'a', 'x', '\022', '\021', '\n', '\r', 'S', 'Y', 'N', 'T', 'A', 'X', '_', 'P', 'R', 'O', 'T', 'O', '2', '\020', '\000', '\022', 
'\021', '\n', '\r', 'S', 'Y', 'N', 'T', 'A', 'X', '_', 'P', 'R', 'O', 'T', 'O', '3', '\020', '\001', '\022', '\023', '\n', '\017', 'S', 'Y', 'N', 
'T', 'A', 'X', '_', 'E', 'D', 'I', 'T', 'I', 'O', 'N', 'S', '\020', '\002', 'B', '{', '\n', '\023', 'c', 'o', 'm', '.', 'g', 'o', 'o', 
'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', '\t', 'T', 'y', 'p', 'e', 'P', 'r', 'o', 't', 'o', 'P', '\001', 
'Z', '-', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'g', 'o', 'l', 'a', 'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 'r', 'o', 't', 'o', 
'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 's', '/', 'k', 'n', 'o', 'w', 'n', '/', 't', 'y', 'p', 'e', 'p', 'b', '\370', '\001', '\001', 
'\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', '\036', 'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 
'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 'n', 'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_type_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/type.proto")) return;
  google_protobuf_any_proto_AddDescriptor();
  google_protobuf_source_context_proto_AddDescriptor();
  DescriptorPool_AddDescriptor("google/protobuf/type.proto", google_protobuf_type_proto_descriptor,
                               sizeof(google_protobuf_type_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_Type, initOnce) {
  google_protobuf_type_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_Type_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_Type, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_Type_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\Type",
                   GPBMetadata_Google_Protobuf_Type_methods);

  GPBMetadata_Google_Protobuf_Type_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_Type */

zend_class_entry* google_protobuf_Type_ce;

static PHP_METHOD(google_protobuf_Type, __construct) {
  google_protobuf_type_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Type, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Type, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Type, getFields) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "fields");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Type, setFields) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "fields");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Type, getOneofs) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "oneofs");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Type, setOneofs) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "oneofs");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Type, getOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Type, setOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Type, getSourceContext) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "source_context");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Type, setSourceContext) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "source_context");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Type, getSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Type, setSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Type, getEdition) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "edition");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Type, setEdition) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "edition");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Type_phpmethods[] = {
  PHP_ME(google_protobuf_Type, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, getFields, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, setFields, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, getOneofs, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, setOneofs, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, getOptions, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, setOptions, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, getSourceContext, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, setSourceContext, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, getSyntax, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, setSyntax, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, getEdition, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Type, setEdition, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Type_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Type",
                   google_protobuf_Type_phpmethods);

  google_protobuf_Type_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Type_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Type_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Type_ce, message_ce);
}

/* google_protobuf_Field */

zend_class_entry* google_protobuf_Field_ce;

static PHP_METHOD(google_protobuf_Field, __construct) {
  google_protobuf_type_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Field, getKind) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "kind");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setKind) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "kind");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getCardinality) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "cardinality");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setCardinality) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "cardinality");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getNumber) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "number");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setNumber) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "number");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "type_url");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setTypeUrl) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "type_url");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getOneofIndex) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "oneof_index");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setOneofIndex) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "oneof_index");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getPacked) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "packed");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setPacked) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "packed");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getJsonName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "json_name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setJsonName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "json_name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Field, getDefaultValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "default_value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Field, setDefaultValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "default_value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Field_phpmethods[] = {
  PHP_ME(google_protobuf_Field, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getKind, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setKind, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getCardinality, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setCardinality, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getNumber, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setNumber, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getTypeUrl, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setTypeUrl, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getOneofIndex, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setOneofIndex, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getPacked, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setPacked, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getOptions, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setOptions, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getJsonName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setJsonName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, getDefaultValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Field, setDefaultValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Field_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Field",
                   google_protobuf_Field_phpmethods);

  google_protobuf_Field_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Field_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Field_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Field_ce, message_ce);
}

/* google_protobuf_Field_Kind */

zend_class_entry* google_protobuf_Field_Kind_ce;

PHP_METHOD(google_protobuf_Field_Kind, name) {
  google_protobuf_type_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.Field.Kind");
  zend_long value;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &value) ==
      FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev =
      upb_EnumDef_FindValueByNumber(e, value);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\Field\\Kind has no name "
                            "defined for value " ZEND_LONG_FMT ".",
                            value);
    return;
  }
  RETURN_STRING(upb_EnumValueDef_Name(ev));
}

PHP_METHOD(google_protobuf_Field_Kind, value) {
  google_protobuf_type_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.Field.Kind");
  char *name = NULL;
  size_t name_len;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &name,
                            &name_len) == FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev = upb_EnumDef_FindValueByNameWithSize(
      e, name, name_len);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\Field\\Kind has no value "
                            "defined for name %s.",
                            name);
    return;
  }
  RETURN_LONG(upb_EnumValueDef_Number(ev));
}

static zend_function_entry google_protobuf_Field_Kind_phpmethods[] = {
  PHP_ME(google_protobuf_Field_Kind, name, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  PHP_ME(google_protobuf_Field_Kind, value, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void google_protobuf_Field_Kind_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Field\\Kind",
                   google_protobuf_Field_Kind_phpmethods);

  google_protobuf_Field_Kind_ce = zend_register_internal_class(&tmp_ce);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_UNKNOWN",
                                   strlen("TYPE_UNKNOWN"), 0);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_DOUBLE",
                                   strlen("TYPE_DOUBLE"), 1);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_FLOAT",
                                   strlen("TYPE_FLOAT"), 2);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_INT64",
                                   strlen("TYPE_INT64"), 3);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_UINT64",
                                   strlen("TYPE_UINT64"), 4);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_INT32",
                                   strlen("TYPE_INT32"), 5);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_FIXED64",
                                   strlen("TYPE_FIXED64"), 6);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_FIXED32",
                                   strlen("TYPE_FIXED32"), 7);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_BOOL",
                                   strlen("TYPE_BOOL"), 8);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_STRING",
                                   strlen("TYPE_STRING"), 9);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_GROUP",
                                   strlen("TYPE_GROUP"), 10);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_MESSAGE",
                                   strlen("TYPE_MESSAGE"), 11);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_BYTES",
                                   strlen("TYPE_BYTES"), 12);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_UINT32",
                                   strlen("TYPE_UINT32"), 13);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_ENUM",
                                   strlen("TYPE_ENUM"), 14);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_SFIXED32",
                                   strlen("TYPE_SFIXED32"), 15);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_SFIXED64",
                                   strlen("TYPE_SFIXED64"), 16);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_SINT32",
                                   strlen("TYPE_SINT32"), 17);
  zend_declare_class_constant_long(google_protobuf_Field_Kind_ce, "TYPE_SINT64",
                                   strlen("TYPE_SINT64"), 18);
}

/* google_protobuf_Field_Cardinality */

zend_class_entry* google_protobuf_Field_Cardinality_ce;

PHP_METHOD(google_protobuf_Field_Cardinality, name) {
  google_protobuf_type_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.Field.Cardinality");
  zend_long value;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &value) ==
      FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev =
      upb_EnumDef_FindValueByNumber(e, value);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\Field\\Cardinality has no name "
                            "defined for value " ZEND_LONG_FMT ".",
                            value);
    return;
  }
  RETURN_STRING(upb_EnumValueDef_Name(ev));
}

PHP_METHOD(google_protobuf_Field_Cardinality, value) {
  google_protobuf_type_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.Field.Cardinality");
  char *name = NULL;
  size_t name_len;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &name,
                            &name_len) == FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev = upb_EnumDef_FindValueByNameWithSize(
      e, name, name_len);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\Field\\Cardinality has no value "
                            "defined for name %s.",
                            name);
    return;
  }
  RETURN_LONG(upb_EnumValueDef_Number(ev));
}

static zend_function_entry google_protobuf_Field_Cardinality_phpmethods[] = {
  PHP_ME(google_protobuf_Field_Cardinality, name, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  PHP_ME(google_protobuf_Field_Cardinality, value, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void google_protobuf_Field_Cardinality_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Field\\Cardinality",
                   google_protobuf_Field_Cardinality_phpmethods);

  google_protobuf_Field_Cardinality_ce = zend_register_internal_class(&tmp_ce);
  zend_declare_class_constant_long(google_protobuf_Field_Cardinality_ce, "CARDINALITY_UNKNOWN",
                                   strlen("CARDINALITY_UNKNOWN"), 0);
  zend_declare_class_constant_long(google_protobuf_Field_Cardinality_ce, "CARDINALITY_OPTIONAL",
                                   strlen("CARDINALITY_OPTIONAL"), 1);
  zend_declare_class_constant_long(google_protobuf_Field_Cardinality_ce, "CARDINALITY_REQUIRED",
                                   strlen("CARDINALITY_REQUIRED"), 2);
  zend_declare_class_constant_long(google_protobuf_Field_Cardinality_ce, "CARDINALITY_REPEATED",
                                   strlen("CARDINALITY_REPEATED"), 3);
}

/* google_protobuf_Enum */

zend_class_entry* google_protobuf_Enum_ce;

static PHP_METHOD(google_protobuf_Enum, __construct) {
  google_protobuf_type_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Enum, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Enum, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Enum, getEnumvalue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "enumvalue");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Enum, setEnumvalue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "enumvalue");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Enum, getOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Enum, setOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Enum, getSourceContext) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "source_context");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Enum, setSourceContext) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "source_context");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Enum, getSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Enum, setSyntax) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "syntax");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Enum, getEdition) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "edition");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Enum, setEdition) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "edition");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Enum_phpmethods[] = {
  PHP_ME(google_protobuf_Enum, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, getEnumvalue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, setEnumvalue, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, getOptions, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, setOptions, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, getSourceContext, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, setSourceContext, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, getSyntax, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, setSyntax, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, getEdition, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Enum, setEdition, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Enum_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Enum",
                   google_protobuf_Enum_phpmethods);

  google_protobuf_Enum_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Enum_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Enum_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Enum_ce, message_ce);
}

/* google_protobuf_EnumValue */

zend_class_entry* google_protobuf_EnumValue_ce;

static PHP_METHOD(google_protobuf_EnumValue, __construct) {
  google_protobuf_type_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_EnumValue, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_EnumValue, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_EnumValue, getNumber) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "number");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_EnumValue, setNumber) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "number");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_EnumValue, getOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_EnumValue, setOptions) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "options");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_EnumValue_phpmethods[] = {
  PHP_ME(google_protobuf_EnumValue, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_EnumValue, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_EnumValue, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_EnumValue, getNumber, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_EnumValue, setNumber, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_EnumValue, getOptions, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_EnumValue, setOptions, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_EnumValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\EnumValue",
                   google_protobuf_EnumValue_phpmethods);

  google_protobuf_EnumValue_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_EnumValue_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_EnumValue_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_EnumValue_ce, message_ce);
}

/* google_protobuf_Option */

zend_class_entry* google_protobuf_Option_ce;

static PHP_METHOD(google_protobuf_Option, __construct) {
  google_protobuf_type_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Option, getName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Option, setName) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "name");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static PHP_METHOD(google_protobuf_Option, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Option, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Option_phpmethods[] = {
  PHP_ME(google_protobuf_Option, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Option, getName, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Option, setName, arginfo_setter, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Option, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Option, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Option_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Option",
                   google_protobuf_Option_phpmethods);

  google_protobuf_Option_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Option_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Option_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Option_ce, message_ce);
}

/* google_protobuf_Syntax */

zend_class_entry* google_protobuf_Syntax_ce;

PHP_METHOD(google_protobuf_Syntax, name) {
  google_protobuf_type_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.Syntax");
  zend_long value;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "l", &value) ==
      FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev =
      upb_EnumDef_FindValueByNumber(e, value);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\Syntax has no name "
                            "defined for value " ZEND_LONG_FMT ".",
                            value);
    return;
  }
  RETURN_STRING(upb_EnumValueDef_Name(ev));
}

PHP_METHOD(google_protobuf_Syntax, value) {
  google_protobuf_type_proto_AddDescriptor();
  const upb_DefPool *symtab = DescriptorPool_GetSymbolTable();
  const upb_EnumDef *e = upb_DefPool_FindEnumByName(symtab, "google.protobuf.Syntax");
  char *name = NULL;
  size_t name_len;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &name,
                            &name_len) == FAILURE) {
    return;
  }
  const upb_EnumValueDef* ev = upb_EnumDef_FindValueByNameWithSize(
      e, name, name_len);
  if (!ev) {
    zend_throw_exception_ex(NULL, 0,
                            "Google\\Protobuf\\Syntax has no value "
                            "defined for name %s.",
                            name);
    return;
  }
  RETURN_LONG(upb_EnumValueDef_Number(ev));
}

static zend_function_entry google_protobuf_Syntax_phpmethods[] = {
  PHP_ME(google_protobuf_Syntax, name, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  PHP_ME(google_protobuf_Syntax, value, arginfo_lookup, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void google_protobuf_Syntax_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Syntax",
                   google_protobuf_Syntax_phpmethods);

  google_protobuf_Syntax_ce = zend_register_internal_class(&tmp_ce);
  zend_declare_class_constant_long(google_protobuf_Syntax_ce, "SYNTAX_PROTO2",
                                   strlen("SYNTAX_PROTO2"), 0);
  zend_declare_class_constant_long(google_protobuf_Syntax_ce, "SYNTAX_PROTO3",
                                   strlen("SYNTAX_PROTO3"), 1);
  zend_declare_class_constant_long(google_protobuf_Syntax_ce, "SYNTAX_EDITIONS",
                                   strlen("SYNTAX_EDITIONS"), 2);
}

/* google/protobuf/wrappers.proto */

zend_class_entry* GPBMetadata_Google_Protobuf_Wrappers_ce;

const char google_protobuf_wrappers_proto_descriptor [455] = {
'\n', '\036', 'g', 'o', 'o', 'g', 'l', 'e', '/', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 'w', 'r', 'a', 'p', 'p', 'e', 'r', 
's', '.', 'p', 'r', 'o', 't', 'o', '\022', '\017', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', '\"', 
'\034', '\n', '\013', 'D', 'o', 'u', 'b', 'l', 'e', 'V', 'a', 'l', 'u', 'e', '\022', '\r', '\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\001', 
' ', '\001', '(', '\001', '\"', '\033', '\n', '\n', 'F', 'l', 'o', 'a', 't', 'V', 'a', 'l', 'u', 'e', '\022', '\r', '\n', '\005', 'v', 'a', 'l', 
'u', 'e', '\030', '\001', ' ', '\001', '(', '\002', '\"', '\033', '\n', '\n', 'I', 'n', 't', '6', '4', 'V', 'a', 'l', 'u', 'e', '\022', '\r', '\n', 
'\005', 'v', 'a', 'l', 'u', 'e', '\030', '\001', ' ', '\001', '(', '\003', '\"', '\034', '\n', '\013', 'U', 'I', 'n', 't', '6', '4', 'V', 'a', 'l', 
'u', 'e', '\022', '\r', '\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\001', ' ', '\001', '(', '\004', '\"', '\033', '\n', '\n', 'I', 'n', 't', '3', 
'2', 'V', 'a', 'l', 'u', 'e', '\022', '\r', '\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\001', ' ', '\001', '(', '\005', '\"', '\034', '\n', '\013', 
'U', 'I', 'n', 't', '3', '2', 'V', 'a', 'l', 'u', 'e', '\022', '\r', '\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\001', ' ', '\001', '(', 
'\r', '\"', '\032', '\n', '\t', 'B', 'o', 'o', 'l', 'V', 'a', 'l', 'u', 'e', '\022', '\r', '\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\001', 
' ', '\001', '(', '\010', '\"', '\034', '\n', '\013', 'S', 't', 'r', 'i', 'n', 'g', 'V', 'a', 'l', 'u', 'e', '\022', '\r', '\n', '\005', 'v', 'a', 
'l', 'u', 'e', '\030', '\001', ' ', '\001', '(', '\t', '\"', '\033', '\n', '\n', 'B', 'y', 't', 'e', 's', 'V', 'a', 'l', 'u', 'e', '\022', '\r', 
'\n', '\005', 'v', 'a', 'l', 'u', 'e', '\030', '\001', ' ', '\001', '(', '\014', 'B', '\203', '\001', '\n', '\023', 'c', 'o', 'm', '.', 'g', 'o', 'o', 
'g', 'l', 'e', '.', 'p', 'r', 'o', 't', 'o', 'b', 'u', 'f', 'B', '\r', 'W', 'r', 'a', 'p', 'p', 'e', 'r', 's', 'P', 'r', 'o', 
't', 'o', 'P', '\001', 'Z', '1', 'g', 'o', 'o', 'g', 'l', 'e', '.', 'g', 'o', 'l', 'a', 'n', 'g', '.', 'o', 'r', 'g', '/', 'p', 
'r', 'o', 't', 'o', 'b', 'u', 'f', '/', 't', 'y', 'p', 'e', 's', '/', 'k', 'n', 'o', 'w', 'n', '/', 'w', 'r', 'a', 'p', 'p', 
'e', 'r', 's', 'p', 'b', '\370', '\001', '\001', '\242', '\002', '\003', 'G', 'P', 'B', '\252', '\002', '\036', 'G', 'o', 'o', 'g', 'l', 'e', '.', 'P', 
'r', 'o', 't', 'o', 'b', 'u', 'f', '.', 'W', 'e', 'l', 'l', 'K', 'n', 'o', 'w', 'n', 'T', 'y', 'p', 'e', 's', 'b', '\006', 'p', 
'r', 'o', 't', 'o', '3', 
};

static void google_protobuf_wrappers_proto_AddDescriptor() {
  if (DescriptorPool_HasFile("google/protobuf/wrappers.proto")) return;
  DescriptorPool_AddDescriptor("google/protobuf/wrappers.proto", google_protobuf_wrappers_proto_descriptor,
                               sizeof(google_protobuf_wrappers_proto_descriptor));
}

static PHP_METHOD(GPBMetadata_Google_Protobuf_Wrappers, initOnce) {
  google_protobuf_wrappers_proto_AddDescriptor();
}

static zend_function_entry GPBMetadata_Google_Protobuf_Wrappers_methods[] = {
  PHP_ME(GPBMetadata_Google_Protobuf_Wrappers, initOnce, arginfo_void, ZEND_ACC_PUBLIC | ZEND_ACC_STATIC)
  ZEND_FE_END
};

static void GPBMetadata_Google_Protobuf_Wrappers_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "GPBMetadata\\Google\\Protobuf\\Wrappers",
                   GPBMetadata_Google_Protobuf_Wrappers_methods);

  GPBMetadata_Google_Protobuf_Wrappers_ce = zend_register_internal_class(&tmp_ce);
}

/* google_protobuf_DoubleValue */

zend_class_entry* google_protobuf_DoubleValue_ce;

static PHP_METHOD(google_protobuf_DoubleValue, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_DoubleValue, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_DoubleValue, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_DoubleValue_phpmethods[] = {
  PHP_ME(google_protobuf_DoubleValue, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_DoubleValue, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_DoubleValue, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_DoubleValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\DoubleValue",
                   google_protobuf_DoubleValue_phpmethods);

  google_protobuf_DoubleValue_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_DoubleValue_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_DoubleValue_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_DoubleValue_ce, message_ce);
}

/* google_protobuf_FloatValue */

zend_class_entry* google_protobuf_FloatValue_ce;

static PHP_METHOD(google_protobuf_FloatValue, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_FloatValue, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_FloatValue, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_FloatValue_phpmethods[] = {
  PHP_ME(google_protobuf_FloatValue, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_FloatValue, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_FloatValue, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_FloatValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\FloatValue",
                   google_protobuf_FloatValue_phpmethods);

  google_protobuf_FloatValue_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_FloatValue_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_FloatValue_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_FloatValue_ce, message_ce);
}

/* google_protobuf_Int64Value */

zend_class_entry* google_protobuf_Int64Value_ce;

static PHP_METHOD(google_protobuf_Int64Value, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Int64Value, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Int64Value, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Int64Value_phpmethods[] = {
  PHP_ME(google_protobuf_Int64Value, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Int64Value, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Int64Value, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Int64Value_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Int64Value",
                   google_protobuf_Int64Value_phpmethods);

  google_protobuf_Int64Value_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Int64Value_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Int64Value_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Int64Value_ce, message_ce);
}

/* google_protobuf_UInt64Value */

zend_class_entry* google_protobuf_UInt64Value_ce;

static PHP_METHOD(google_protobuf_UInt64Value, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_UInt64Value, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_UInt64Value, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_UInt64Value_phpmethods[] = {
  PHP_ME(google_protobuf_UInt64Value, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_UInt64Value, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_UInt64Value, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_UInt64Value_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\UInt64Value",
                   google_protobuf_UInt64Value_phpmethods);

  google_protobuf_UInt64Value_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_UInt64Value_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_UInt64Value_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_UInt64Value_ce, message_ce);
}

/* google_protobuf_Int32Value */

zend_class_entry* google_protobuf_Int32Value_ce;

static PHP_METHOD(google_protobuf_Int32Value, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_Int32Value, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_Int32Value, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_Int32Value_phpmethods[] = {
  PHP_ME(google_protobuf_Int32Value, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Int32Value, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_Int32Value, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_Int32Value_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\Int32Value",
                   google_protobuf_Int32Value_phpmethods);

  google_protobuf_Int32Value_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_Int32Value_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_Int32Value_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_Int32Value_ce, message_ce);
}

/* google_protobuf_UInt32Value */

zend_class_entry* google_protobuf_UInt32Value_ce;

static PHP_METHOD(google_protobuf_UInt32Value, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_UInt32Value, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_UInt32Value, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_UInt32Value_phpmethods[] = {
  PHP_ME(google_protobuf_UInt32Value, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_UInt32Value, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_UInt32Value, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_UInt32Value_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\UInt32Value",
                   google_protobuf_UInt32Value_phpmethods);

  google_protobuf_UInt32Value_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_UInt32Value_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_UInt32Value_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_UInt32Value_ce, message_ce);
}

/* google_protobuf_BoolValue */

zend_class_entry* google_protobuf_BoolValue_ce;

static PHP_METHOD(google_protobuf_BoolValue, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_BoolValue, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_BoolValue, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_BoolValue_phpmethods[] = {
  PHP_ME(google_protobuf_BoolValue, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_BoolValue, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_BoolValue, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_BoolValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\BoolValue",
                   google_protobuf_BoolValue_phpmethods);

  google_protobuf_BoolValue_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_BoolValue_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_BoolValue_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_BoolValue_ce, message_ce);
}

/* google_protobuf_StringValue */

zend_class_entry* google_protobuf_StringValue_ce;

static PHP_METHOD(google_protobuf_StringValue, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_StringValue, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_StringValue, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_StringValue_phpmethods[] = {
  PHP_ME(google_protobuf_StringValue, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_StringValue, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_StringValue, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_StringValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\StringValue",
                   google_protobuf_StringValue_phpmethods);

  google_protobuf_StringValue_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_StringValue_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_StringValue_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_StringValue_ce, message_ce);
}

/* google_protobuf_BytesValue */

zend_class_entry* google_protobuf_BytesValue_ce;

static PHP_METHOD(google_protobuf_BytesValue, __construct) {
  google_protobuf_wrappers_proto_AddDescriptor();
  zim_Message___construct(INTERNAL_FUNCTION_PARAM_PASSTHRU);
}

static PHP_METHOD(google_protobuf_BytesValue, getValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval ret;
  Message_get(intern, f, &ret);
  RETURN_COPY_VALUE(&ret);
}

static PHP_METHOD(google_protobuf_BytesValue, setValue) {
  Message* intern = (Message*)Z_OBJ_P(getThis());
  const upb_FieldDef *f = upb_MessageDef_FindFieldByName(
      intern->desc->msgdef, "value");
  zval *val;
  if (zend_parse_parameters(ZEND_NUM_ARGS(), "z", &val)
      == FAILURE) {
    return;
  }
  Message_set(intern, f, val);
  RETURN_COPY(getThis());
}

static zend_function_entry google_protobuf_BytesValue_phpmethods[] = {
  PHP_ME(google_protobuf_BytesValue, __construct, arginfo_construct, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_BytesValue, getValue, arginfo_void, ZEND_ACC_PUBLIC)
  PHP_ME(google_protobuf_BytesValue, setValue, arginfo_setter, ZEND_ACC_PUBLIC)
  ZEND_FE_END
};

static void google_protobuf_BytesValue_ModuleInit() {
  zend_class_entry tmp_ce;

  INIT_CLASS_ENTRY(tmp_ce, "Google\\Protobuf\\BytesValue",
                   google_protobuf_BytesValue_phpmethods);

  google_protobuf_BytesValue_ce = zend_register_internal_class(&tmp_ce);
  google_protobuf_BytesValue_ce->ce_flags |= ZEND_ACC_FINAL;
  google_protobuf_BytesValue_ce->create_object = Message_create;
  zend_do_inheritance(google_protobuf_BytesValue_ce, message_ce);
}

static void WellKnownTypes_ModuleInit() {
  GPBMetadata_Google_Protobuf_Any_ModuleInit();
  google_protobuf_Any_ModuleInit();
  GPBMetadata_Google_Protobuf_Api_ModuleInit();
  google_protobuf_Api_ModuleInit();
  google_protobuf_Method_ModuleInit();
  google_protobuf_Mixin_ModuleInit();
  GPBMetadata_Google_Protobuf_Duration_ModuleInit();
  google_protobuf_Duration_ModuleInit();
  GPBMetadata_Google_Protobuf_GPBEmpty_ModuleInit();
  google_protobuf_Empty_ModuleInit();
  GPBMetadata_Google_Protobuf_FieldMask_ModuleInit();
  google_protobuf_FieldMask_ModuleInit();
  GPBMetadata_Google_Protobuf_SourceContext_ModuleInit();
  google_protobuf_SourceContext_ModuleInit();
  GPBMetadata_Google_Protobuf_Struct_ModuleInit();
  google_protobuf_Struct_ModuleInit();
  google_protobuf_Struct_FieldsEntry_ModuleInit();
  google_protobuf_Value_ModuleInit();
  google_protobuf_ListValue_ModuleInit();
  google_protobuf_NullValue_ModuleInit();
  GPBMetadata_Google_Protobuf_Timestamp_ModuleInit();
  google_protobuf_Timestamp_ModuleInit();
  GPBMetadata_Google_Protobuf_Type_ModuleInit();
  google_protobuf_Type_ModuleInit();
  google_protobuf_Field_ModuleInit();
  google_protobuf_Field_Kind_ModuleInit();
  google_protobuf_Field_Cardinality_ModuleInit();
  google_protobuf_Enum_ModuleInit();
  google_protobuf_EnumValue_ModuleInit();
  google_protobuf_Option_ModuleInit();
  google_protobuf_Syntax_ModuleInit();
  GPBMetadata_Google_Protobuf_Wrappers_ModuleInit();
  google_protobuf_DoubleValue_ModuleInit();
  google_protobuf_FloatValue_ModuleInit();
  google_protobuf_Int64Value_ModuleInit();
  google_protobuf_UInt64Value_ModuleInit();
  google_protobuf_Int32Value_ModuleInit();
  google_protobuf_UInt32Value_ModuleInit();
  google_protobuf_BoolValue_ModuleInit();
  google_protobuf_StringValue_ModuleInit();
  google_protobuf_BytesValue_ModuleInit();
}
