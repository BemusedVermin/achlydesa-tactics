import sys, glob, heapq, math, json
sys.path.insert(0,'tools/scripts')
import numpy as np, rasterio, hillshade as h
W={'A':(30.0,34.6),'B':(29.0,35.0),'C':(28.3,33.5),'D':(31.8,36.5)}
tiles=glob.glob('data/raw/terrain/*.tif')
PX=500  # 300 m cells
CELL=150000/PX
out={}
for k,(la,lo) in W.items():
    e=h.sample_window(np,rasterio,tiles,la,lo,150.0,PX)  # row0 north
    e=np.nan_to_num(e,nan=float(np.nanmedian(e)))
    gy,gx=np.gradient(e,CELL)
    slope=np.degrees(np.arctan(np.hypot(gx,gy)))
    np.save(f'{k}_e.npy',e); np.save(f'{k}_s.npy',slope)

    print(k,'slope p50/p90',np.percentile(slope,[50,90]).round(1),'passable(<8deg)',(slope<8).mean().round(2))
