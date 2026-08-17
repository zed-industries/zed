#ifndef Logger_H
#define Logger_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "Logger.d.h"






bool icu4x_Logger_init_simple_logger_mv1(void);


void icu4x_Logger_destroy_mv1(Logger* self);





#endif // Logger_H
