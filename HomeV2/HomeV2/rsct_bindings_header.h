//
//  rsct_bindings_header.h
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/09/2023.
//

#ifndef rsct_bindings_header_h
#define rsct_bindings_header_h

#include <stdint.h>
#include <stdbool.h>
struct CameraData {
    void * data;
    size_t length;
    size_t __cap;
    bool success;
};

void * create_new_scheduler();

void * create_new_reassembler();

// Function to create a server and return a void pointer to it.
void * create_server(const char * port, size_t portlen, void * sched, size_t width, size_t height);

// Function to listen on the server and return a Vec<u8> as a char array.
// The caller is responsible for freeing the memory allocated for the char array.
struct CameraData listen_once(void * raw_server, void * sched, void * reassembler, uint64_t timeout);

struct CameraData send_connection_ping(void * raw_server, void * sched, const char * client_string);

void drop_camera_data(struct CameraData data);

void drop_configuration(void *, void *, void *);

#endif /* rsct_bindings_header_h */
