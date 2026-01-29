use std::ffi::c_void;
use std::sync::OnceLock;
use windows::core::HRESULT;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D11::D3D11_CREATE_DEVICE_FLAG;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL};
use windows::Win32::Graphics::Dxgi::{Common, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG};

pub type D3D11CreateDeviceAndSwapChain = unsafe extern "system" fn(
    padapter: *mut c_void,
    drivertype: D3D_DRIVER_TYPE,
    software: HMODULE,
    flags: D3D11_CREATE_DEVICE_FLAG,
    pfeaturelevels: *const D3D_FEATURE_LEVEL,
    featurelevels: u32,
    sdkversion: u32,
    pswapchaindesc: *const DXGI_SWAP_CHAIN_DESC,
    ppswapchain: *mut *mut IDXGISwapChain,
    ppdevice: *mut *mut c_void,
    pfeaturelevel: *mut D3D_FEATURE_LEVEL,
    ppimmediatecontext: *mut *mut c_void,
) -> HRESULT;

pub type PresentFn = unsafe extern "system" fn(IDXGISwapChain, u32, u32) -> i32; // *mut IDXGISwapChain
pub type ResizeBuffersFn = unsafe extern "system" fn(
    *mut IDXGISwapChain,
    u32,
    u32,
    u32,
    Common::DXGI_FORMAT,
    DXGI_SWAP_CHAIN_FLAG,
);

pub static ORIGINAL_DEV_CHAIN: OnceLock<D3D11CreateDeviceAndSwapChain> = OnceLock::new();
pub static ORIGINAL_PRESENT: OnceLock<PresentFn> = OnceLock::new();
pub static ORIGINAL_RESIZE_BUFFERS: OnceLock<ResizeBuffersFn> = OnceLock::new();
