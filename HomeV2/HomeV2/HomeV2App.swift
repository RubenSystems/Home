//
//  HomeV2App.swift
//  HomeV2
//
//  Created by Ruben Ticehurst-James on 06/09/2023.
//

import SwiftUI
import UserNotifications



@main
struct HomeV2App: App {
    
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate

    
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}



class AppDelegate: NSObject, UIApplicationDelegate {
    
    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey : Any]? = nil) -> Bool {
        application.applicationIconBadgeNumber = 0
        let authOptions: UNAuthorizationOptions = [.alert, .badge, .sound]
        UNUserNotificationCenter.current().requestAuthorization(options: authOptions, completionHandler: {_, _ in })
        application.registerForRemoteNotifications()
        return true
    }

    func application(_ application: UIApplication, didRegisterForRemoteNotificationsWithDeviceToken deviceToken: Data) {
        let tokenParts = deviceToken.map { data in String(format: "%02.2hhx", data) }
        let token = tokenParts.joined()
        print("TOKEN", token)
        
    }
    
    func application(_ application: UIApplication, didReceiveRemoteNotification userInfo: [AnyHashable : Any]) {
        print(userInfo)
    }
}
