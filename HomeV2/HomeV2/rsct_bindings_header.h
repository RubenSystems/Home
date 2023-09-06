//
//  rsct_bindings_header.h
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/09/2023.
//

#ifndef rsct_bindings_header_h
#define rsct_bindings_header_h

struct Server;

// Function to create a server and return a void pointer to it.
void* create_server(const char* port);

// Function to listen on the server and return a Vec<u8> as a char array.
// The caller is responsible for freeing the memory allocated for the char array.
char* listen(void* raw_server);

#endif /* rsct_bindings_header_h */
