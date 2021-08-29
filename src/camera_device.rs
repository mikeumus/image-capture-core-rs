use cocoa::base::{id, BOOL};
use cocoa::foundation::{NSTimeInterval, NSUInteger};
use libc::off_t;
use objc::*;
use crate::device::{ICDevice, ICDeviceType};
use core_graphics::image::CGImageRef;
use libc::{c_int, c_longlong};

pub trait ICCameraDevice {
    /// Get the delegate.
    unsafe fn delegate(self) -> id;
    /// Set the delegate.
    unsafe fn setDelegate(self, delegate: id);
    /// Indicates if the device has reported battery charge level￼.
    unsafe fn batteryLevelAvailable(self) -> BOOL;
    /// Indicates the battery charge level. Its value ranges from 0 to 100.
    unsafe fn batteryLevel(self) -> NSUInteger;
    /// Indicates the percentage of content cataloging completed on the device. Its value ranges from 0 to 100.
    unsafe fn contentCatalogPercentCompleted(self) -> NSUInteger;
    /// Contents of the camera. The structure of the elements in this array will reflect the folder structure of the storage reported by the camera.
    /// Each item in this array will correspond to a storage on the camera.
    unsafe fn contents(self) -> id;
    /// The property mediaFiles represents all image, movie and audio files on the camera.
    /// These files are returned as a single array without regard to the folder hierarchy used to store these files on the camera.
    unsafe fn mediaFiles(self) -> id;
    /// Indicates the time offset, in seconds, between the camera's clock and the computer's clock￼.
    /// This value is positive if the camera's clock is ahead of the computer's clock.
    /// This property should be ignored if the camera's capabilities property does not contain ICCameraDeviceCanSyncClock.
    unsafe fn timeOffset(self) -> NSTimeInterval;
    /// Set to YES if the device is made by Apple and is pass-coded locked and connected to an untrusted host.
    unsafe fn isAccessRestrictedAppleDevice(self) -> BOOL;
    /// Filesystem mount point for a device with transportType of ICTransportTypeMassStorage.
    /// This will be NULL for all other devices.
    unsafe fn mountPoint(self) -> id;
    /// This property is set to YES when tethered capture is enabled on the device.
    unsafe fn tetheredCaptureEnabled(self) -> BOOL;
    /// This method returns an array of files on the camera of type fileType.
    unsafe fn filesOfType(self, fileUTType: id) -> id;
    /// Synchronize camera's clock with the computer's clock.
    /// You should send this request only if the camera has the 'ICCameraDeviceCanSyncClock' capability.
    unsafe fn requestSyncClock(self);
    /// Send this message to enable tethered capture on the camera device if the camera has the 'ICCameraDeviceCanTakePicture' capability.
    unsafe fn requestEnableTethering(self);
    /// Send this message to disable tethered capture on the camera device if the camera has the 'ICCameraDeviceCanTakePicture' capability and if your process has already sent a 'requestEnableTethering' to it.
    unsafe fn requestDisableTethering(self);
    /// Capture a new image using the camera, the camera capabilities include 'ICCameraDeviceCanTakePicture'.
    unsafe fn requestTakePicture(self);
    /// Deletes files.
    unsafe fn requestDeleteFiles(self, files: id);
    /// Cancels the current delete operation started by sending a 'requestDeleteFiles:'.
    unsafe fn cancelDelete(self);
    /// Download a file from the camera. Please refer to the top of this header for information about the options.
    unsafe fn requestDownloadFile(
        self,
        file: id,
        options: id,
        downloadDelegate: id,
        didDownloadSelector: id,
        contextInfo: id,
    );
    /// Cancels the current download operation.
    unsafe fn cancelDownload(self);
    /// Upload a file at fileURL to the camera. The options dictionary is not used in this version.
    unsafe fn requestUploadFile(
        self,
        fileURL: id,
        options: id,
        uploadDelegate: id,
        didUploadSelector: id,
        contextInfo: id,
    );
    /// This method asynchronously reads data of a specified length from a specified offset.
    unsafe fn requestReadDataFromFile(
        self,
        file: id,
        atOffset: off_t,
        length: off_t,
        readDelegate: id,
        didReadDataSelector: id,
        contextInfo: id,
    );
    /// This method asynchronously sends a PTP command to a camera.
    unsafe fn requestSendPTPCommand(
        self,
        command: id,
        outData: id,
        sendCommandDelegate: id,
        didSendCommandSelector: id,
        contextInfo: id,
    );
}

impl std::fmt::Debug for &dyn ICCameraDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}",  self )
    }
}

impl ICCameraDevice for &dyn ICCameraDevice {
    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate(self, delegate: id) {
        // msg_send![self, setDelegate: delegate]
        ICDevice::setDelegate(self, delegate)
    }

    unsafe fn batteryLevelAvailable(self) -> BOOL {
        msg_send![self, batteryLevelAvailable]
    }

    unsafe fn batteryLevel(self) -> NSUInteger {
        msg_send![self, batteryLevel]
    }

    unsafe fn contentCatalogPercentCompleted(self) -> NSUInteger {
        msg_send![self, contentCatalogPercentCompleted]
    }

    unsafe fn contents(self) -> id {
        msg_send![self, contents]
    }

    unsafe fn mediaFiles(self) -> id {
        msg_send![self, mediaFiles]
    }

    unsafe fn timeOffset(self) -> NSTimeInterval {
        msg_send![self, timeOffset]
    }

    unsafe fn isAccessRestrictedAppleDevice(self) -> BOOL {
        msg_send![self, isAccessRestrictedAppleDevice]
    }

    unsafe fn mountPoint(self) -> id {
        msg_send![self, mountPoint]
    }

    unsafe fn tetheredCaptureEnabled(self) -> BOOL {
        msg_send![self, tetheredCaptureEnabled]
    }

    unsafe fn filesOfType(self, fileUTType: id) -> id {
        msg_send![self, filesOfType: fileUTType]
    }

    unsafe fn requestSyncClock(self) {
        msg_send![self, requestSyncClock]
    }

    unsafe fn requestEnableTethering(self) {
        msg_send![self, requestEnableTethering]
    }

    unsafe fn requestDisableTethering(self) {
        msg_send![self, requestDisableTethering]
    }

    unsafe fn requestTakePicture(self) {
        msg_send![self, requestTakePicture]
    }

    unsafe fn requestDeleteFiles(self, files: id) {
        msg_send![self, requestDeleteFiles: files]
    }

    unsafe fn cancelDelete(self) {
        msg_send![self, cancelDelete]
    }

    unsafe fn requestDownloadFile(
        self,
        file: id,
        options: id,
        downloadDelegate: id,
        didDownloadSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestDownloadFile:file options:options downloadDelegate:downloadDelegate didDownloadSelector:didDownloadSelector contextInfo:contextInfo]
    }

    unsafe fn cancelDownload(self) {
        msg_send![self, cancelDownload]
    }

    unsafe fn requestUploadFile(
        self,
        fileURL: id,
        options: id,
        uploadDelegate: id,
        didUploadSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestUploadFile:fileURL options:options uploadDelegate:uploadDelegate didUploadSelector:didUploadSelector contextInfo:contextInfo]
    }

    unsafe fn requestReadDataFromFile(
        self,
        file: id,
        atOffset: off_t,
        length: off_t,
        readDelegate: id,
        didReadDataSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestReadDataFromFile:file atOffset:atOffset length:length readDelegate:readDelegate didReadDataSelector:didReadDataSelector contextInfo:contextInfo]
    }

    unsafe fn requestSendPTPCommand(
        self,
        command: id,
        outData: id,
        sendCommandDelegate: id,
        didSendCommandSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestSendPTPCommand:command outData:outData sendCommandDelegate:sendCommandDelegate didSendCommandSelector:didSendCommandSelector contextInfo:contextInfo]
    }
}

impl ICDevice for &dyn ICCameraDevice {
    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate(self, delegate: id) {
        msg_send![self, setDelegate: delegate]
    }

    unsafe fn type_(self) -> ICDeviceType {
        msg_send![self, type]
    }

    unsafe fn name(self) -> id {
        msg_send![self, name]
    }

    unsafe fn icon(self) -> CGImageRef {
        msg_send![self, icon]
    }

    unsafe fn capabilities(self) -> id {
        msg_send![self, capabilities]
    }

    unsafe fn modulePath(self) -> id {
        msg_send![self, modulePath]
    }

    unsafe fn moduleVersion(self) -> id {
        msg_send![self, moduleVersion]
    }

    unsafe fn isRemote(self) -> BOOL {
        msg_send![self, isRemote]
    }

    unsafe fn transportType(self) -> id {
        msg_send![self, transportType]
    }

    unsafe fn usbLocationID(self) -> c_int {
        msg_send![self, usbLocationID]
    }

    unsafe fn usbProductID(self) -> c_int {
        msg_send![self, usbProductID]
    }

    unsafe fn usbVendorID(self) -> c_int {
        msg_send![self, usbVendorID]
    }

    unsafe fn fwGUID(self) -> c_longlong {
        msg_send![self, fwGUID]
    }

    unsafe fn serialNumberString(self) -> id {
        msg_send![self, serialNumberString]
    }

    unsafe fn locationDescription(self) -> id {
        msg_send![self, locationDescription]
    }

    unsafe fn hasOpenSession(self) -> BOOL {
        msg_send![self, hasOpenSession]
    }

    unsafe fn UUIDString(self) -> id {
        msg_send![self, UUIDString]
    }

    unsafe fn persistentIDString(self) -> id {
        msg_send![self, persistentIDString]
    }

    unsafe fn buttonPressed(self) -> id {
        msg_send![self, buttonPressed]
    }

    unsafe fn autolaunchApplicationPath(self) -> id {
        msg_send![self, autolaunchApplicationPath]
    }

    unsafe fn setAutolaunchApplicationPath(self, autolaunchApplicationPath: id) {
        msg_send![
            self,
            setAutolaunchApplicationPath: autolaunchApplicationPath
        ]
    }

    unsafe fn userData(self) -> id {
        msg_send![self, userData]
    }

    unsafe fn requestOpenSession(self) {
        msg_send![self, requestOpenSession]
    }

    unsafe fn requestCloseSession(self) {
        msg_send![self, requestCloseSession]
    }

    unsafe fn requestYield(self) {
        msg_send![self, requestYield]
    }

    unsafe fn requestSendMessage(
        self,
        messageCode: u64,
        outData: id,
        maxReturnedDataSize: u64,
        sendMessageDelegate: id,
        didSendMessageSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestSendMessage:messageCode outData:outData maxReturnedDataSize:maxReturnedDataSize sendMessageDelegate:sendMessageDelegate didSendMessageSelector:didSendMessageSelector contextInfo:contextInfo]
    }

    unsafe fn requestEjectOrDisconnect(self) {
        msg_send![self, requestEjectOrDisconnect]
    }
}

impl ICCameraDevice for id {
    unsafe fn delegate(self) -> id {
        msg_send![self, delegate]
    }

    unsafe fn setDelegate(self, delegate: id) {
        // msg_send![self, setDelegate: delegate]
        ICDevice::setDelegate(self, delegate)
    }

    unsafe fn batteryLevelAvailable(self) -> BOOL {
        msg_send![self, batteryLevelAvailable]
    }

    unsafe fn batteryLevel(self) -> NSUInteger {
        msg_send![self, batteryLevel]
    }

    unsafe fn contentCatalogPercentCompleted(self) -> NSUInteger {
        msg_send![self, contentCatalogPercentCompleted]
    }

    unsafe fn contents(self) -> id {
        msg_send![self, contents]
    }

    unsafe fn mediaFiles(self) -> id {
        msg_send![self, mediaFiles]
    }

    unsafe fn timeOffset(self) -> NSTimeInterval {
        msg_send![self, timeOffset]
    }

    unsafe fn isAccessRestrictedAppleDevice(self) -> BOOL {
        msg_send![self, isAccessRestrictedAppleDevice]
    }

    unsafe fn mountPoint(self) -> id {
        msg_send![self, mountPoint]
    }

    unsafe fn tetheredCaptureEnabled(self) -> BOOL {
        msg_send![self, tetheredCaptureEnabled]
    }

    unsafe fn filesOfType(self, fileUTType: id) -> id {
        msg_send![self, filesOfType: fileUTType]
    }

    unsafe fn requestSyncClock(self) {
        msg_send![self, requestSyncClock]
    }

    unsafe fn requestEnableTethering(self) {
        msg_send![self, requestEnableTethering]
    }

    unsafe fn requestDisableTethering(self) {
        msg_send![self, requestDisableTethering]
    }

    unsafe fn requestTakePicture(self) {
        msg_send![self, requestTakePicture]
    }

    unsafe fn requestDeleteFiles(self, files: id) {
        msg_send![self, requestDeleteFiles: files]
    }

    unsafe fn cancelDelete(self) {
        msg_send![self, cancelDelete]
    }

    unsafe fn requestDownloadFile(
        self,
        file: id,
        options: id,
        downloadDelegate: id,
        didDownloadSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestDownloadFile:file options:options downloadDelegate:downloadDelegate didDownloadSelector:didDownloadSelector contextInfo:contextInfo]
    }

    unsafe fn cancelDownload(self) {
        msg_send![self, cancelDownload]
    }

    unsafe fn requestUploadFile(
        self,
        fileURL: id,
        options: id,
        uploadDelegate: id,
        didUploadSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestUploadFile:fileURL options:options uploadDelegate:uploadDelegate didUploadSelector:didUploadSelector contextInfo:contextInfo]
    }

    unsafe fn requestReadDataFromFile(
        self,
        file: id,
        atOffset: off_t,
        length: off_t,
        readDelegate: id,
        didReadDataSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestReadDataFromFile:file atOffset:atOffset length:length readDelegate:readDelegate didReadDataSelector:didReadDataSelector contextInfo:contextInfo]
    }

    unsafe fn requestSendPTPCommand(
        self,
        command: id,
        outData: id,
        sendCommandDelegate: id,
        didSendCommandSelector: id,
        contextInfo: id,
    ) {
        msg_send![self, requestSendPTPCommand:command outData:outData sendCommandDelegate:sendCommandDelegate didSendCommandSelector:didSendCommandSelector contextInfo:contextInfo]
    }
}

#[link(name = "ImageCaptureCore", kind = "framework")]
extern "C" {
    // Constants used to describe capabilities of a camera. (NSString *const)

    /// Indicates that the device uses USB transport
    pub static ICTransportTypeUSB: id;
    /// Indicates that the camera can capture a picture while it is connected, if the client sends a 'requestTakePicture' message to it.
    pub static ICCameraDeviceCanTakePicture: id;
    /// Indicates that the camera can capture a picture while it is connected, if the user presses the shutter release on the camera.
    pub static ICCameraDeviceCanTakePictureUsingShutterReleaseOnCamera: id;
    /// Indicates that the camera can delete a file at a time while it is connected.
    pub static ICCameraDeviceCanDeleteOneFile: id;
    /// Indicates that the camera can delete all files in a single operation while it is connected.
    pub static ICCameraDeviceCanDeleteAllFiles: id;
    /// Indicates that the camera can synchronize its date and time with that of the host computer.
    pub static ICCameraDeviceCanSyncClock: id;
    /// Indicates that the host can upload files to the camera.
    pub static ICCameraDeviceCanReceiveFile: id;
    /// Indicates that the camera can accept PTP commands.
    pub static ICCameraDeviceCanAcceptPTPCommands: id;

    // Allowed keys in the options dictionary used when downloading a file from the camera

    /// The value for this key should be an NSURL object referencing a writable directory. The downloaded files will be saved in that directory.
    pub static ICDownloadsDirectoryURL: id;
    /// The value for this key should be an NSString object containing the name to be used for the downloaded file.
    pub static ICSaveAsFilename: id;
    /// The value for this key will be an NSString object containing the actual name of the saved file. The options dictionary returned in didDownloadFile:error:options:contextInfo: will have this key.
    pub static ICSavedFilename: id;
    /// The value for this key will be an NSArray object containing names of files associated with the primary file that is downloaded. The options dictionary returned in didDownloadFile:error:options:contextInfo: may have this key.
    pub static ICSavedAncillaryFiles: id;
    /// The value for this key should be an NSNumber object representing a boolean value. If this value is YES, the downloaded file will overwrite an existing file with the same name and extension.
    pub static ICOverwrite: id;
    /// The value for this key should be an NSNumber object representing a boolean value. If this value is YES, the file will be deleted from the device after it is succcessfully downloaded.
    pub static ICDeleteAfterSuccessfulDownload: id;
    /// The value for this key should be an NSNumber object representing a boolean value. If this value is YES, all sidecar files will be downloaded along with the media file.
    pub static ICDownloadSidecarFiles: id;
}
