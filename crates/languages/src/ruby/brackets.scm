("[" @open "]" @close)
("{" @open "}" @close)
("\"" @open "\"" @close)
("do" @open "end" @close)

(block_parameters "|" @open "|" @close)
(interpolation "#{" @open "}" @close)

(if "if" @open "end" @close)
(unless "unless" @open "end" @close)
(begin "begin" @open "end" @close)
(module "module" @open "end" @close)
(_ . "def" @open "end" @close)
(_ . "class" @open "end" @close)