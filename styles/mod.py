import os, sys, re


def camel_to_snake(inputstring):
    REG = r'(?<!^)(?=[A-Z])'
    return re.sub(REG, '_', inputstring).lower()


def change_case(mypath):
    if os.path.isabs(mypath):
        raise ValueError
    else:
        abs_path_to_dir = os.path.normpath(os.path.join(os.path.dirname(__file__), mypath))
        with os.scandir(abs_path_to_dir) as iter:
            dirs = []
            typescriptfiles = []
            for entry in iter:
                if (entry.is_dir() and entry.name not in ["node_modules", "target"]):
                    dirs.append(entry.name)
                if (entry.is_file() and entry.name.endswith('.ts')):
                    typescriptfiles.append(entry.name)
            if len(dirs) != 0:
                for dir in dirs:
                    change_case(os.path.normpath(os.path.join(mypath,dir)))
            for entry in typescriptfiles:
                relative_path = os.path.normpath(os.path.join(mypath,entry))
                dst = camel_to_snake(relative_path)
                abs_path = os.path.normpath(os.path.join(os.path.dirname(__file__), relative_path))
                abs_dst = os.path.normpath(os.path.join(os.path.dirname(__file__), dst))
                (head, tail) = os.path.split(abs_dst)
                if not os.path.exists(head):
                    os.makedirs(head)
                os.rename(abs_path, abs_dst)

def main():
    dir = os.path.dirname(__file__)
    path = sys.argv[1]
    change_case(path)


if __name__ == '__main__':
    main()
