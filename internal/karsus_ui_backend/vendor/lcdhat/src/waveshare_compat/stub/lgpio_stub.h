#ifndef LCDHAT_LGPIO_STUB_H
#define LCDHAT_LGPIO_STUB_H

#include <pthread.h>

#define LG_HIGH 1
#define LG_LOW 0
#define LG_SET_INPUT 0

static inline int lgGpiochipOpen(int chip) { (void)chip; return 0; }
static inline int lgGpiochipClose(int handle) { (void)handle; return 0; }
static inline int lgGpioClaimInput(int handle, int flags, int pin) { (void)handle; (void)flags; (void)pin; return 0; }
static inline int lgGpioClaimOutput(int handle, int flags, int pin, int level)
{
    (void)handle;
    (void)flags;
    (void)pin;
    (void)level;
    return 0;
}
static inline int lgGpioWrite(int handle, int pin, int level) { (void)handle; (void)pin; (void)level; return 0; }
static inline int lgGpioRead(int handle, int pin) { (void)handle; (void)pin; return 1; }

static inline int lgSpiOpen(int spiDev, int spiChan, int baud, int flags)
{
    (void)spiDev;
    (void)spiChan;
    (void)baud;
    (void)flags;
    return 0;
}
static inline int lgSpiClose(int spi) { (void)spi; return 0; }
static inline int lgSpiWrite(int spi, char *txBuf, int count) { (void)spi; (void)txBuf; return count; }

static inline void lguSleep(double seconds) { (void)seconds; }

static inline pthread_t *lgThreadStart(void *(*f)(void *), void *userdata)
{
    (void)f;
    (void)userdata;
    return (pthread_t *)0;
}

#endif
