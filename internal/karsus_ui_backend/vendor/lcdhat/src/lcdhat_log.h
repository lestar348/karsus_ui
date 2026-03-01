#ifndef LCDHAT_LOG_H
#define LCDHAT_LOG_H

#include <stdarg.h>

void lcdhat_log_init(void);
void lcdhat_log_close(void);
void lcdhat_log_error(const char *fmt, ...);
void lcdhat_log_debug(const char *fmt, ...);

#endif
