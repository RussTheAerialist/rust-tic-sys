// #include <libpololu-tic-1/tic.h>
#include <stddef.h>

struct tic_device;
struct tic_error;

const char * tic_error_get_message(const struct tic_error *);
void tic_error_free(struct tic_error *);

struct tic_error * tic_list_connected_devices(
  struct tic_device *** device_list,
  size_t * device_count);
void tic_list_free(struct tic_device ** list);

void tic_device_free(struct tic_device *);
const char * tic_device_get_name(const struct tic_device *);
const char * tic_device_get_serial_number(const struct tic_device *);