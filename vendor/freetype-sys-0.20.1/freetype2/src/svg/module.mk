#
# FreeType 2 SVG renderer module definition
#


# Copyright (C) 2022-2023 by
# David Turner, Robert Wilhelm, Werner Lemberg, and Moazin Khatti.
#
# This file is part of the FreeType project, and may only be used, modified,
# and distributed under the terms of the FreeType project license,
# LICENSE.TXT.  By continuing to use, modify, or distribute this file you
# indicate that you have read the license and understand and accept it
# fully.


FTMODULE_H_COMMANDS += SVG_MODULE

define SVG_MODULE
$(OPEN_DRIVER) FT_Renderer_Class, ft_svg_renderer_class $(CLOSE_DRIVER)
$(ECHO_DRIVER)ot-svg    $(ECHO_DRIVER_DESC)OT-SVG glyph renderer module$(ECHO_DRIVER_DONE)
endef

# EOF
