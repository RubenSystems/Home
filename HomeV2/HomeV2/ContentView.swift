//
//  ContentView.swift
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/09/2023.
//

import SwiftUI


struct ContentView: View {

    
    var body: some View {
        VStack {
            LivestreamView(camera_address: "192.168.86.245:5253")
        }

    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
