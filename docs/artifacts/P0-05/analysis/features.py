import sys, json, math, numpy as np
sys.path.insert(0,'.')
from path import *
WP={'A':[(58,2),(62,40),(70,100),(75,148)],
    'B':[(75,2),(75,70),(78,100),(80,148)],
    'C':[(15,2),(28,50),(38,75),(30,100),(30,148)],
    'D':[(75,2),(75,75),(75,148)]}
def cell(p): return (min(N-1,max(0,int(N-1-p[1]/0.3))), min(N-1,max(0,int(p[0]/0.3))))
def km(r,q): return (q+0.5)*0.3, (N-r-0.5)*0.3
out={}
for k in 'ABCD':
    e=np.load(f'{k}_e.npy'); s=np.load(f'{k}_s.npy'); acc=np.load(f'{k}_acc.npy'); c=cost_grid(e,s); c=c+6.0*(np.load(f'{k}_acc.npy')>150)
    sea=np.abs(e)<0.5
    path=[]
    for a,b in zip(WP[k],WP[k][1:]):
        d,prev=dijkstra(c,[cell(a)]); path+=trace(prev,*cell(b))
    pk=np.array([km(r,q) for r,q in path]); seg=np.hypot(*np.diff(pk,axis=0).T); cum=np.r_[0,np.cumsum(seg)]
    L=cum[-1]; sl=np.array([s[p] for p in path]); el=np.array([e[p] for p in path])
    res=dict(road_km=round(L,1),mean_slope=round(float(sl.mean()),1),max_slope=round(float(sl.max()),1),elev_range=(int(el.min()),int(el.max())))
    # crossings
    accn=np.maximum.reduce([np.roll(np.roll(acc,dr,0),dc,1) for dr in (-1,0,1) for dc in (-1,0,1)])
    cross={}
    for T in (150,400,1000):
        runs=[];cur=[]
        for i,p in enumerate(path):
            if accn[p]>T and not sea[p]: cur.append(i)
            else:
                if cur: runs.append(cur); cur=[]
        if cur: runs.append(cur)
        cr=[]
        for r_ in runs:
            ln=(cum[r_[-1]]-cum[r_[0]])+0.3
            if ln<=1.5:
                m=float(np.mean(cum[r_])); cr.append((round(m,1),int(max(accn[path[i]] for i in r_)),[round(float(v),1) for v in pk[r_[len(r_)//2]]]))
        cross[T]=cr
    res['crossings']=cross
    # best 20 km window with most crossings (T=150)
    cs=[c_[0] for c_ in cross[400]]; best=(0,None)
    for a in cs:
        n=sum(1 for b in cs if a<=b<=a+20)
        if n>best[0]: best=(n,a)
    res['three_crossings_best']=best
    # reedbank: crossing with acc 400-8000
    res['reedbank_candidates']=[c_ for c_ in cross[400] if 1000<=c_[1]<=20000]
    # escarpment: cells with slope>15 within 12 km of road
    rows,cols=np.where((s>15)&~sea); P=np.array([km(r,q) for r,q in zip(rows,cols)]) if len(rows) else np.zeros((0,2))
    sub=pk[::10]
    if len(P):
        dm=np.sqrt(((P[:,None,:]-sub[None,:,:])**2).sum(-1)).min(1); near=P[dm<12]
        res['escarp_cells_within12km']=int(len(near)); 
        if len(near): res['escarp_centroid']=[round(float(v),1) for v in near.mean(0)]; res['escarp_min_dist_km']=round(float(dm.min()),1); res['escarp_extent_x']=(round(float(near[:,0].min()),0),round(float(near[:,0].max()),0)); res['escarp_extent_y']=(round(float(near[:,1].min()),0),round(float(near[:,1].max()),0))
    # high ground: >=p90 elev comps >= 100 km2 not within 10 km of road
    thr=np.percentile(e[~sea],90); hi=(e>=thr)&~sea; seen=np.zeros_like(hi); comps=[]
    for r0,q0 in zip(*np.where(hi)):
        if seen[r0,q0]: continue
        st=[(r0,q0)]; seen[r0,q0]=True; mem=[]
        while st:
            r,q=st.pop(); mem.append((r,q))
            for dr,dc in D8:
                r2,q2=r+dr,q+dc
                if 0<=r2<N and 0<=q2<N and hi[r2,q2] and not seen[r2,q2]: seen[r2,q2]=True; st.append((r2,q2))
        if len(mem)>=1100: comps.append(mem)
    hc=[]
    for m in comps:
        pts=np.array([km(r,q) for r,q in m]); cen=pts.mean(0); dist=float(np.sqrt(((sub-cen)**2).sum(1)).min())
        hc.append((len(m)*0.09,[round(float(v),1) for v in cen],int(max(e[r,q] for r,q in m)),round(dist,1)))
    res['high_ground_p90']=int(thr); res['high_comps(km2,centroid,maxelev,dist_to_road)']=sorted(hc,reverse=True)[:4]
    out[k]=res; out[k]['path']=[[round(float(x),1),round(float(y),1)] for x,y in pk[::10]]
    print(k,json.dumps({kk:v for kk,v in res.items() if kk!='crossings' or True},default=str))
json.dump(out,open('features.json','w'))
