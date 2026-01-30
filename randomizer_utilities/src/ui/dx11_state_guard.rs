use std::slice;
use windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY;
use windows::Win32::Graphics::Direct3D11::*;

pub struct DX11OverlayBackup {
    rtvs: [Option<ID3D11RenderTargetView>; 8],
    dsv: Option<ID3D11DepthStencilView>,

    viewport: D3D11_VIEWPORT,
    rs_state: Option<ID3D11RasterizerState>,

    blend_state: Option<ID3D11BlendState>,
    blend_factor: [f32; 4],
    sample_mask: u32,

    depth_state: Option<ID3D11DepthStencilState>,
    stencil_ref: u32,

    input_layout: Option<ID3D11InputLayout>,
    topology: D3D_PRIMITIVE_TOPOLOGY,

    constant_buffer: Option<ID3D11Buffer>,
}

impl DX11OverlayBackup {
    /// Backup all the bits before we render
    pub fn new(ctx: &ID3D11DeviceContext) -> Self {
        unsafe {
            let mut rtvs: [Option<ID3D11RenderTargetView>; 8] = Default::default();
            let mut dsv = None;
            ctx.OMGetRenderTargets(Some(&mut rtvs), Some(&mut dsv));

            let mut viewport = D3D11_VIEWPORT::default();
            let mut viewport_count = 1;
            ctx.RSGetViewports(&mut viewport_count, Some(&mut viewport));

            let rs_state = ctx.RSGetState().ok();

            let mut blend_state = None;
            let mut blend_factor = [0.0; 4];
            let mut sample_mask = 0;
            ctx.OMGetBlendState(
                Some(&mut blend_state),
                Some(&mut blend_factor),
                Some(&mut sample_mask),
            );

            // Depth-stencil
            let mut depth_state = None;
            let mut stencil_ref = 0;
            ctx.OMGetDepthStencilState(Some(&mut depth_state), Some(&mut stencil_ref));

            // IA
            let input_layout = ctx.IAGetInputLayout().ok();
            let topology = ctx.IAGetPrimitiveTopology();

            let mut constant_buffer = None;
            ctx.PSGetConstantBuffers(0, Some(slice::from_mut(&mut constant_buffer)));

            Self {
                rtvs,
                dsv,
                viewport,
                rs_state,
                blend_state,
                blend_factor,
                sample_mask,
                depth_state,
                stencil_ref,
                input_layout,
                topology,
                constant_buffer,
            }
        }
    }

    /// Restore once the overlay has finished
    pub fn restore(self, ctx: &ID3D11DeviceContext) {
        unsafe {
            ctx.OMSetRenderTargets(Some(&self.rtvs), self.dsv.as_ref());

            ctx.RSSetViewports(Some(slice::from_ref(&self.viewport)));
            ctx.RSSetState(self.rs_state.as_ref());

            ctx.OMSetBlendState(
                self.blend_state.as_ref(),
                Some(&self.blend_factor),
                self.sample_mask,
            );
            ctx.OMSetDepthStencilState(self.depth_state.as_ref(), self.stencil_ref);

            ctx.IASetInputLayout(self.input_layout.as_ref());
            ctx.IASetPrimitiveTopology(self.topology);

            ctx.PSSetShader(None, None);
            ctx.PSSetShaderResources(0, Some(&[const { None }; 8]));
            ctx.PSSetConstantBuffers(0, Some(slice::from_ref(&self.constant_buffer)));
            ctx.PSSetSamplers(0, Some(&[const { None }; 8]));
        }
    }
}
