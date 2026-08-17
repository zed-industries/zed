#!/usr/bin/env python
"""
Rust code generator, based on neovim-qt generator
"""

import msgpack
import sys, subprocess, os
import re
import jinja2
import datetime

INPUT = 'bindings'

MANUALLY_IMPLEMENTED = [
        "nvim_ui_attach",
        "nvim_tabpage_list_wins",
        "nvim_tabpage_get_win",
        "nvim_win_get_buf",
        "nvim_win_get_tabpage",
        "nvim_list_bufs",
        "nvim_get_current_buf",
        "nvim_list_wins",
        "nvim_get_current_win",
        "nvim_create_buf",
        "nvim_open_win",
        "nvim_list_tabpages",
        "nvim_get_current_tabpage",
        ]

def decutf8(inp):
    """
    Recursively decode bytes as utf8 into unicode
    """
    if isinstance(inp, bytes):
        return inp.decode('utf8')
    elif isinstance(inp, list):
        return [decutf8(x) for x in inp]
    elif isinstance(inp, dict):
        return {decutf8(key):decutf8(val) for key,val in inp.items()}
    else:
        return inp

def get_api_info(nvim):
    """
    Call the neovim binary to get the api info
    """
    args = [nvim, '--api-info']
    info = subprocess.check_output(args)
    return decutf8(msgpack.unpackb(info))

def generate_file(name, outpath, **kw):
    from jinja2 import Environment, FileSystemLoader
    env=Environment(loader=FileSystemLoader('bindings'), trim_blocks=True)
    template = env.get_template(name)
    with open(os.path.join(outpath, name), 'w') as fp:
        fp.write(template.render(kw))

    subprocess.call(["rustfmt", os.path.join(outpath, name)])
    # os.remove(os.path.join(outpath, name + ".bk"))

class UnsupportedType(Exception): pass
class NeovimTypeVal:
    """
    Representation for Neovim Parameter/Return
    """
    # msgpack simple types types
    SIMPLETYPES_REF = {
            'Array': 'Vec<Value>',
            'ArrayOf(Integer, 2)': '(i64, i64)',
            'void': '()',
            'Integer': 'i64',
            'Float': 'f64',
            'Boolean': 'bool',
            'String': '&str',
            'Object': 'Value',
            'Dict': 'Vec<(Value, Value)>',
        }

    SIMPLETYPES_VAL = {
            'Array': 'Vec<Value>',
            'ArrayOf(Integer, 2)': '(i64, i64)',
            'void': '()',
            'Integer': 'i64',
            'Float': 'f64',
            'Boolean': 'bool',
            'String': 'String',
            'Object': 'Value',
            'Dict': 'Vec<(Value, Value)>',
        }
    # msgpack extension types
    EXTTYPES = {
            'Window': 'Window<W>',
            'Buffer': 'Buffer<W>',
            'Tabpage': 'Tabpage<W>',
        }
    # Unbound Array types
    UNBOUND_ARRAY = re.compile(r'ArrayOf\(\s*(\w+)\s*\)')

    def __init__(self, typename, name=''):
        self.name = name
        self.neovim_type = typename
        self.ext = False
        self.native_type_arg = NeovimTypeVal.nativeTypeRef(typename)
        self.native_type_ret = NeovimTypeVal.nativeTypeVal(typename)

        if typename in self.EXTTYPES:
            self.ext = True

    def __getitem__(self, key):
        if key == "native_type_arg":
            return self.native_type_arg
        if key == "name":
            return self._convert_arg_name(self.name)
        return None

    def _convert_arg_name(self, key):
        """Rust keyword must not be used as function arguments"""
        if key == "fn":
            return "fname"
        if key == "type":
            return "typ"
        return key

    @classmethod
    def nativeTypeVal(cls, typename):
        """Return the native type for this Neovim type."""
        if typename in cls.SIMPLETYPES_VAL:
            return cls.SIMPLETYPES_VAL[typename]
        elif typename in cls.EXTTYPES:
            return cls.EXTTYPES[typename]
        elif cls.UNBOUND_ARRAY.match(typename):
            m = cls.UNBOUND_ARRAY.match(typename)
            return 'Vec<%s>' % cls.nativeTypeVal(m.groups()[0])
        raise UnsupportedType(typename)


    @classmethod
    def nativeTypeRef(cls, typename):
        """Return the native type for this Neovim type."""
        if typename in cls.SIMPLETYPES_REF:
            return cls.SIMPLETYPES_REF[typename]
        elif typename in cls.EXTTYPES:
            return "&%s" % cls.EXTTYPES[typename]
        elif cls.UNBOUND_ARRAY.match(typename):
            m = cls.UNBOUND_ARRAY.match(typename)
            return 'Vec<%s>' % cls.nativeTypeVal(m.groups()[0])
        raise UnsupportedType(typename)

class Function:
    """
    Representation for a Neovim API Function
    """
    def __init__(self, nvim_fun, all_ext_prefixes):
        self.valid = False
        self.fun = nvim_fun
        self.parameters = []
        self.name =  self.fun['name']
        self.since = self.fun['since']

        self.ext = self._is_ext(all_ext_prefixes)

        try:
            self.return_type = NeovimTypeVal(self.fun['return_type'])
            if self.ext:
                for param in self.fun['parameters'][1:]:
                    self.parameters.append(NeovimTypeVal(*param))
            else:
                for param in self.fun['parameters']:
                    self.parameters.append(NeovimTypeVal(*param))
        except UnsupportedType as ex:
            print('Found unsupported type(%s) when adding function %s(), skipping' % (ex,self.name))
            return

        # Build the argument string - makes it easier for the templates
        self.argstring = ', '.join(['%s: %s' % (tv["name"], tv.native_type_arg) for tv in self.parameters])

        # Build the call string - even easier for the templates ;)
        self.callstring = ', '.join(['%s' % tv["name"] for tv in self.parameters])
        # filter function, use only nvim one
        # nvim_ui_attach implemented manually
        self.valid = self.name.startswith('nvim')\
                and self.name not in MANUALLY_IMPLEMENTED

    def _is_ext(self, all_ext_prefixes):
        for prefix in all_ext_prefixes:
            if self.name.startswith(prefix):
                return True
        return False

class ExtType:

    """Ext type, Buffer, Window, Tab"""

    def __init__(self, typename, info):
        self.name = typename
        self.id = info['id']
        self.prefix = info['prefix']


def print_api(api):
    print(api.keys());
    for key in api.keys():
        if key == 'functions':
            print('Functions')
            for f in api[key]:
                if f['name'].startswith('nvim'):
                    print(f)
            print('')
        elif key == 'types':
            print('Data Types')
            for typ in api[key]:
                print('\t%s' % typ)
            print('')
        elif key == 'error_types':
            print('Error Types')
            for err,desc in api[key].items():
                print('\t%s:%d' % (err,desc['id']))
            print('')
        elif key == 'version':
            print('Version')
            print(api[key])
            print('')
        else:
            print('Unknown API info attribute: %s' % key)

if __name__ == '__main__':

    if len(sys.argv) < 2 or len(sys.argv) > 3 :
        print('Usage:')
        print('\tgenerate_bindings <nvim>')
        print('\tgenerate_bindings <nvim> [path]')
        sys.exit(-1)

    nvim = sys.argv[1]
    outpath = None if len(sys.argv) < 3 else sys.argv[2]

    try:
        api = get_api_info(sys.argv[1])
    except subprocess.CalledProcessError as ex:
        print(ex)
        sys.exit(-1)

    if outpath:
        print('Writing auto generated bindings to %s' % outpath)
        if not os.path.exists(outpath):
            os.makedirs(outpath)
        for name in os.listdir(INPUT):
            if name.startswith('.'):
                continue
            if name.endswith('.rs'):
                env = {}
                env['date'] = datetime.datetime.now()

                exttypes = [ ExtType(typename, info) for typename,info in api['types'].items() ]
                all_ext_prefixes = { exttype.prefix for exttype in exttypes }
                functions = [Function(f, all_ext_prefixes) for f in api['functions']]
                env['functions'] = [f for f in functions if f.valid]
                env['exttypes'] = exttypes
                generate_file(name, outpath, **env)

    else:
        print('Neovim api info:')
        print_api(api)
