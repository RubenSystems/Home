//
//  rsct_bindings_header.h
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/09/2023.
//

#ifndef rsct_bindings_header_h
#define rsct_bindings_header_h

#include <stdint.h>

struct CameraData {
    void * data;
    size_t length;
    size_t __cap;
};

void * create_new_scheduler();

// Function to create a server and return a void pointer to it.
void * create_server(const char * port, size_t portlen, void * sched);

// Function to listen on the server and return a Vec<u8> as a char array.
// The caller is responsible for freeing the memory allocated for the char array.
struct CameraData listen_once(void * raw_server, void * sched);

void drop_camera_data(struct CameraData data);

#endif /* rsct_bindings_header_h */
