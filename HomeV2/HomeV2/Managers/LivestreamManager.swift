//
//  LivestreamManager.swift
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/10/2023.
//

import Foundation
import UIKit.UIImage

class LivestreamManager : ObservableObject {

    static private let ping_count_down_tick_count = 100
    
    @Published public var image: UIImage? = nil
    
    struct ConfigInfo {
        let server: UnsafeMutableRawPointer
        let scheduler: UnsafeMutableRawPointer
        let reassembler: UnsafeMutableRawPointer
    }
    
    
    public var config: ConfigInfo? = nil
    public var ping_count_down = ping_count_down_tick_count;
    private var is_livestreaming = false
    private let semaphore = DispatchSemaphore(value: 1)
    
    
    private func configure() {
        let sched = create_new_scheduler()!
        
        self.config = ConfigInfo(
            server: create_server("5254".cString(using: .utf8)!, 4, sched, 2328, 1748),
            scheduler: sched,
            reassembler: create_new_reassembler()
        )
    }
    
    private func tear_down() {
        guard let config =  self.config else {return}
        drop_configuration(config.server, config.scheduler, config.reassembler)
    }
    
    public func start_livestream(camera_address: String) {
        configure()
        self.is_livestreaming = true
        guard let config = self.config else {return}
        
        let _ = DispatchQueue.global().sync {
            send_connection_ping(config.server, config.scheduler, (camera_address as NSString).utf8String)
        }
            
        
        DispatchQueue.global().async{
            while self.is_livestreaming {
                let camera_data = listen_once(config.server, config.scheduler, config.reassembler, 100);
                if !self.is_livestreaming {
                    self.tear_down()
                }
                if camera_data.success == true {
                    self.set_image(to: camera_data)
                }
//                self.ping_count_down -= 1
//                if self.ping_count_down <= 0 {
//                    self.ping_count_down = LivestreamManager.ping_count_down_tick_count
//                    send_connection_ping(config.server, config.scheduler, (camera_address as NSString).utf8String)
//                    print("SENT")
//                }
            }
        }
    }
    
    private func set_image(to camera_data: CameraData) {
        DispatchQueue.main.async {
            let data = Data(bytes: camera_data.data, count: camera_data.length)
            if let image = UIImage(data: data) {
                self.semaphore.wait()
                self.image = image
                self.semaphore.signal()
            } else {
                print("BROKEN_IMAGE_ERROR")
            }
//            drop_camera_data(camera_data)
            
        }
    }
    
    public func stop_livestream() {
        self.is_livestreaming = false
    }
}
