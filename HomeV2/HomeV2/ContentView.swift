//
//  ContentView.swift
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/09/2023.
//

import SwiftUI


class LivestreamManager : ObservableObject {

    @Published public var image: UIImage? = nil
    
    struct ConfigInfo {
        let server: UnsafeMutableRawPointer
        let scheduler: UnsafeMutableRawPointer
    }
    
    private var config: ConfigInfo? = nil
    private var is_livestreaming = false
    private let semaphore = DispatchSemaphore(value: 1)
    
    init() {
        self.configure()
    }
    
    
    private func configure() {
        let sched = create_new_scheduler()!
        
        self.config = ConfigInfo(
            server: create_server("5254".cString(using: .utf8)!, 4, sched),
            scheduler: sched
        )
    }
    
    private func tear_down() {
        print("REMINDER: FREE THE MEMORY!!!")
    }
    
    public func start_livestream() {
        self.is_livestreaming = true
        guard let config = self.config else {return}
        
        DispatchQueue.global().async{
            while self.is_livestreaming {
                let camera_data = listen_once(config.server, config.scheduler);
                self.set_image(to: camera_data)
            }
        }
    }
    
    private func set_image(to camera_data: CameraData) {
        DispatchQueue.main.async {
            let data = Data(bytes: camera_data.data, count: camera_data.length)
            if let image = UIImage(data: data) {
                // enter shared memory
                self.semaphore.wait()
                self.image = image
                self.semaphore.signal()
                // exit shared memory
            }
            drop_camera_data(camera_data)
        }
    }
    
    public func stop_livestream() {
        self.is_livestreaming = false
    }
}

struct ContentView: View {
    var data: Data? = nil
    @StateObject var livestream_manager = LivestreamManager()
    
    
    var body: some View {
        VStack {
            if let image = self.livestream_manager.image {
                Image(uiImage: image)
                    .resizable()
                    .aspectRatio(image.size.width/image.size.height, contentMode: .fit)
                    .cornerRadius(25)
            }
            Spacer()
        }
        .onAppear(perform: self.livestream_manager.start_livestream)
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
