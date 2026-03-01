#include "lcdhat_log.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct {
    int initialized;
    int debug_enabled;
    int use_file;
    FILE *out;
} lcdhat_logger_t;

static lcdhat_logger_t g_logger = {0, 0, 0, NULL};

static int parse_bool_env(const char *value)
{
    size_t i;
    char normalized[16];

    if (value == NULL) {
        return 0;
    }

    while (*value == ' ' || *value == '\t') {
        value++;
    }

    if (*value == '\0') {
        return 0;
    }

    if (strcmp(value, "1") == 0) {
        return 1;
    }

    for (i = 0; i < sizeof(normalized) - 1 && value[i] != '\0'; i++) {
        normalized[i] = (char)tolower((unsigned char)value[i]);
    }
    normalized[i] = '\0';

    return strcmp(normalized, "true") == 0 || strcmp(normalized, "yes") == 0 ||
           strcmp(normalized, "on") == 0 || strcmp(normalized, "debug") == 0;
}

static void write_log_line(const char *level, const char *fmt, va_list args)
{
    time_t raw_time;
    struct tm local_tm;
    char timestamp[32];

    if (g_logger.out == NULL) {
        g_logger.out = stdout;
    }

    raw_time = time(NULL);
#if defined(_POSIX_THREAD_SAFE_FUNCTIONS)
    localtime_r(&raw_time, &local_tm);
#else
    {
        struct tm *tmp = localtime(&raw_time);
        if (tmp != NULL) {
            local_tm = *tmp;
        } else {
            memset(&local_tm, 0, sizeof(local_tm));
        }
    }
#endif
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", &local_tm);

    fprintf(g_logger.out, "[%s] [lcdhat] [%s] ", timestamp, level);
    vfprintf(g_logger.out, fmt, args);
    fputc('\n', g_logger.out);
    fflush(g_logger.out);
}

void lcdhat_log_init(void)
{
    const char *debug_env;
    const char *file_env;

    if (g_logger.initialized) {
        return;
    }

    g_logger.out = stdout;
    g_logger.debug_enabled = 0;
    g_logger.use_file = 0;

    debug_env = getenv("LCDHAT_LOG_DEBUG");
    g_logger.debug_enabled = parse_bool_env(debug_env);

    file_env = getenv("LCDHAT_LOG_FILE");
    if (file_env != NULL && file_env[0] != '\0') {
        FILE *file = fopen(file_env, "a");
        if (file != NULL) {
            g_logger.out = file;
            g_logger.use_file = 1;
        } else {
            fprintf(stdout,
                    "[lcdhat] [ERROR] Cannot open log file '%s'. Falling back to stdout.\n",
                    file_env);
            fflush(stdout);
        }
    }

    g_logger.initialized = 1;

    if (g_logger.debug_enabled) {
        lcdhat_log_debug("Debug logging is enabled (LCDHAT_LOG_DEBUG=%s)",
                         debug_env ? debug_env : "");
    }
}

void lcdhat_log_close(void)
{
    if (!g_logger.initialized) {
        return;
    }

    if (g_logger.use_file && g_logger.out != NULL) {
        fclose(g_logger.out);
    }

    g_logger.initialized = 0;
    g_logger.debug_enabled = 0;
    g_logger.use_file = 0;
    g_logger.out = NULL;
}

void lcdhat_log_error(const char *fmt, ...)
{
    va_list args;

    lcdhat_log_init();
    va_start(args, fmt);
    write_log_line("ERROR", fmt, args);
    va_end(args);
}

void lcdhat_log_debug(const char *fmt, ...)
{
    va_list args;

    lcdhat_log_init();
    if (!g_logger.debug_enabled) {
        return;
    }

    va_start(args, fmt);
    write_log_line("DEBUG", fmt, args);
    va_end(args);
}
