#include <cstddef>

#ifdef __HAIKU__

#include <cstring>

#include <Errors.h>
#include <LocaleRoster.h>
#include <String.h>
#include <TimeZone.h>

extern "C" {

size_t iana_time_zone_haiku_get_tz(char *buf, size_t buf_size) {
    try {
        static_assert(sizeof(char) == sizeof(uint8_t), "Illegal char size");

        if (buf_size == 0) {
            return 0;
        }

        // `BLocaleRoster::Default()` returns a reference to a statically allocated object.
        // https://github.com/haiku/haiku/blob/8f16317/src/kits/locale/LocaleRoster.cpp#L143-L147
        BLocaleRoster *locale_roster(BLocaleRoster::Default());
        if (!locale_roster) {
            return 0;
        }

        BTimeZone tz(NULL, NULL);
        if (locale_roster->GetDefaultTimeZone(&tz) != B_OK) {
            return 0;
        }

        BString bname(tz.ID());
        int32_t ilength(bname.Length());
        if (ilength <= 0) {
            return 0;
        }

        size_t length(ilength);
        if (length > buf_size) {
            return 0;
        }

        // BString::String() returns a borrowed string.
        // https://www.haiku-os.org/docs/api/classBString.html#ae4fe78b06c8e3310093b80305e14ba87
        const char *sname(bname.String());
        if (!sname) {
            return 0;
        }

        std::memcpy(buf, sname, length);
        return length;
    } catch (...) {
        return 0;
    }
}
}  // extern "C"

#else

extern "C" {

size_t iana_time_zone_haiku_get_tz(char *buf, size_t buf_size) { return 0; }
}  // extern "C"

#endif
