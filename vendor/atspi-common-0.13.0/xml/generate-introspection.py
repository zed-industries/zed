import xml.etree.ElementTree as ET
tree = ET.parse("../xml/Event.xml")
root = tree.getroot()

RUST_TYPE_FROM_DBUS_TYPE = {
  "s": "String",
  "i": "i32",
  "u": "u32",
  "v": "&zvariant::OwnedValue",
  "a{sv}": "&std::collections::HashMap<String, zvariant::OwnedValue>",
  "o": "OwnedPath"
}
ARG_IDENT_FROM_NUMBER = [
  "kind",
  "detail1",
  "detail2",
  "any_data",
  "properties"
]

# will store lists of members with their various details. Will output a markdown table.
interfaces = []

def print_interface_table(signals):
  print(f"/// Event table for the contained types:\n///")
  print(f"/// Interface|Member|Kind|Detail 1|Detail 2|Any Data|Properties")
  print(f"/// |:--|---|---|---|---|---|---|")
  for signal in signals:
    interface = signal["interface"].replace("Event", "")
    member = signal["member"]
    kind = signal["kind"]["name"] or "    "
    detail1 = signal["detail1"]["name"] or "    "
    detail2 = signal["detail2"]["name"] or "    "
    data = signal["any_data"]["name"] or "    "
    properties = signal["properties"]["name"]
    print(f"/// |{interface}|{member}|{kind}|{detail1}|{detail2}|{data}|{properties}|")

def impl_functions(signal):
  iface = signal["interface"].replace("Event", "")
  impl_member = signal["member"]
  print(f"impl {impl_member}Event {{")
  for (zbus_name, struct_name) in signal.items():
    if zbus_name in ARG_IDENT_FROM_NUMBER and struct_name["name"]:
      rust_type = RUST_TYPE_FROM_DBUS_TYPE[struct_name["type"]]
      print(f"\t#[must_use]")
      print(f"\tpub fn {struct_name['name']}(&self) -> {rust_type} {{")
      if rust_type == "String":
        print(f"\t\tself.0.{zbus_name}().to_string()")
      else:
        print(f"\t\tself.0.{zbus_name}()")
      print(f"}}")
  print(f"}}")

for interface in root:
  interface_name = interface.attrib["name"].split(".")[-1]
  enum_name = interface_name + "Event"
  print(f"pub mod {interface_name.lower()} {{")
  print("use zbus::zvariant;")

  enum = f"pub enum {enum_name}s {{\n"
# will contain all the attributes to use for markdown table

  signals = []
  structs = []
  for signal in interface:
    signal_identities = dict()
    variant_name = signal.attrib["name"]
    signal_identities["interface"] = enum_name
    signal_identities["member"] = variant_name
    enum += f"\t{variant_name}({variant_name}Event),\n"
    struct = f"//#[derive(Debug, Clone)]\n"
    struct += f"pub struct {variant_name}Event(crate::events::AtspiEvent);"
    for (argi, arg) in enumerate(signal):
# ignore <annotation> tags for QSPI
      if arg.tag != "arg":
        continue
      signal_identities[ARG_IDENT_FROM_NUMBER[argi]] = {"name":None,"type":None}
# skip getting additional info if the name of the arg is not specified (trhis means it is not used)
      if "name" not in arg.attrib:
        continue
      field_name = arg.attrib["name"]
      field_type = arg.attrib["type"]
      rust_type = RUST_TYPE_FROM_DBUS_TYPE[field_type]
      signal_identities[ARG_IDENT_FROM_NUMBER[argi]] = arg.attrib
      # do not put struct items, use methods instead
			#struct += f"\t{field_name}: {rust_type},\n"
    signals.append(signal_identities)
    #struct += "}"
    structs.append(struct)
  enum += "}"
  interfaces.append(signals)
  for (struct, signal) in zip(structs, signals):
    print(struct)
    impl_functions(signal)
  print_interface_table(signals)
  print(enum)
# close the module
  print("}")


