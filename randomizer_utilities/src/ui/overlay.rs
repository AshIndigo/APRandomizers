use std::sync::{LazyLock, OnceLock, RwLock};
use windows::core::PCSTR;
use windows::Win32::Graphics::Direct3D11::{ID3D11Buffer, ID3D11Device, ID3D11DeviceContext, ID3D11InputLayout, ID3D11RenderTargetView};
use windows::Win32::Graphics::Direct3D::Fxc::D3DCompile;
use windows::Win32::Graphics::Direct3D::ID3DBlob;
use crate::ui::font_handler::FontAtlas;

pub struct D3D11State {
    pub device: ID3D11Device,
    pub context: ID3D11DeviceContext,
    pub atlas: Option<FontAtlas>,
    pub input_layout: ID3D11InputLayout,
    pub vertex_buffer: ID3D11Buffer,
    pub rtv: Option<ID3D11RenderTargetView>,
}

pub static STATE: OnceLock<RwLock<D3D11State>> = OnceLock::new();

pub static SHADERS: LazyLock<(ID3DBlob, ID3DBlob)> = LazyLock::new(|| {
    let mut vs_blob: Option<ID3DBlob> = None;
    let mut ps_blob: Option<ID3DBlob> = None;
    let mut err_blob: Option<ID3DBlob> = None;

    let vs_bytes = include_bytes!("./data/text_vs.hlsl");
    let ps_bytes = include_bytes!("./data/text_ps.hlsl");

    unsafe {
        D3DCompile(
            vs_bytes.as_ptr() as *const _,
            vs_bytes.len(),
            None,
            None,
            None,
            PCSTR::from_raw(c"main".as_ptr() as *const u8),
            PCSTR::from_raw(c"vs_5_0".as_ptr() as *const u8),
            0,
            0,
            &mut vs_blob,
            Some(&mut err_blob),
        )
            .expect("Couldn't compile VS");
        D3DCompile(
            ps_bytes.as_ptr() as *const _,
            ps_bytes.len(),
            None,
            None,
            None,
            PCSTR::from_raw(c"main".as_ptr() as *const u8),
            PCSTR::from_raw(c"ps_5_0".as_ptr() as *const u8),
            0,
            0,
            &mut ps_blob,
            Some(&mut err_blob),
        )
            .expect("Couldn't compile PS");
    }
    (ps_blob.unwrap(), vs_blob.unwrap())
});