//
//  LivestreamView.swift
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/10/2023.
//

import SwiftUI

struct LivestreamView: View {
    
    let camera_address : String
    @StateObject var livestream_manager = LivestreamManager()
    @Environment(\.scenePhase) var scenePhase
    
    var body: some View {
        VStack {
            if let image = self.livestream_manager.image {
                Image(uiImage: image)
                    .resizable()
                    .aspectRatio(image.size.width/image.size.height, contentMode: .fit)
                    .cornerRadius(25)
            } else {
                Rectangle()
                    .aspectRatio(1920/1080, contentMode: .fit)
                    .foregroundColor(.gray)
                    .cornerRadius(25)
            }
        }
        .onChange(of: scenePhase) { newPhase in
            switch newPhase {
            case .active:
                
                self.livestream_manager.start_livestream(camera_address: camera_address)
                
                
                break
            case .background, .inactive:
                self.livestream_manager.stop_livestream()
                break
            @unknown default:
                self.livestream_manager.stop_livestream()
                break
            }
        }
    }
}
