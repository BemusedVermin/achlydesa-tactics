import heapq, json, math, numpy as np
N=500; CELL=300.0
D8=[(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]
def cost_grid(e,slope):
    c=1+(slope/6.0)**2
    c[slope>22]=np.inf
    c[np.abs(e)<0.5]=np.inf  # sea
    return c
def dijkstra(c,srcs):
    dist=np.full(c.shape,np.inf); prev=-np.ones(c.shape,dtype=np.int64)
    pq=[]
    for (r,q) in srcs:
        if np.isfinite(c[r,q]): dist[r,q]=0; heapq.heappush(pq,(0.0,r,q))
    while pq:
        d,r,q=heapq.heappop(pq)
        if d>dist[r,q]: continue
        for dr,dc in D8:
            r2,q2=r+dr,q+dc
            if 0<=r2<N and 0<=q2<N and np.isfinite(c[r2,q2]):
                step=math.hypot(dr,dc)*0.5*(c[r,q]+c[r2,q2])
                nd=d+step
                if nd<dist[r2,q2]:
                    dist[r2,q2]=nd; prev[r2,q2]=r*N+q; heapq.heappush(pq,(nd,r2,q2))
    return dist,prev
def trace(prev,r,q):
    p=[]
    while r>=0:
        p.append((r,q)); v=prev[r,q]; r,q=(v//N,v%N) if v>=0 else (-1,-1)
    return p[::-1]
def flowacc(e):
    # priority-flood fill then D8 accumulation
    f=e.copy(); closed=np.zeros(e.shape,bool); pq=[]
    for r in range(N):
        for q in (0,N-1):
            heapq.heappush(pq,(f[r,q],r,q)); closed[r,q]=True
    for q in range(N):
        for r in (0,N-1):
            if not closed[r,q]: heapq.heappush(pq,(f[r,q],r,q)); closed[r,q]=True
    order=[]
    while pq:
        z,r,q=heapq.heappop(pq); order.append((r,q))
        for dr,dc in D8:
            r2,q2=r+dr,q+dc
            if 0<=r2<N and 0<=q2<N and not closed[r2,q2]:
                closed[r2,q2]=True; f[r2,q2]=max(f[r2,q2],z+1e-3*0+1e-4); heapq.heappush(pq,(f[r2,q2],r2,q2))
    acc=np.ones(e.shape)
    for (r,q) in reversed(order):  # high -> low
        best=None;bd=0
        for dr,dc in D8:
            r2,q2=r+dr,q+dc
            if 0<=r2<N and 0<=q2<N:
                d=(f[r,q]-f[r2,q2])/math.hypot(dr,dc)
                if d>bd: bd=d;best=(r2,q2)
        if best: acc[best]+=acc[r,q]
    return acc
if __name__=='__main__':
    res={}
    for k in 'ABCD':
        e=np.load(f'{k}_e.npy'); s=np.load(f'{k}_s.npy'); c=cost_grid(e,s)
        acc=flowacc(e); np.save(f'{k}_acc.npy',acc)
        best=None
        for name,srcs,tg in (('S->N',[(N-1,q) for q in range(N)],lambda d:[(0,q) for q in range(N)]),
                             ('W->E',[(r,0) for r in range(N)],lambda d:[(r,N-1) for r in range(N)])):
            dist,prev=dijkstra(c,srcs)
            tgs=tg(None); r,q=min(tgs,key=lambda t:dist[t])
            p=trace(prev,r,q)
            km=sum(math.hypot(a[0]-b[0],a[1]-b[1]) for a,b in zip(p,p[1:]))*CELL/1000
            sl=[s[a] for a in p]
            mean=float(np.mean(sl)); mx=float(np.max(sl)); p90=float(np.percentile(sl,90))
            print(k,name,f'len {km:.0f} km mean slope {mean:.1f} p90 {p90:.1f} max {mx:.1f} cost/cell {dist[r,q]/len(p):.2f}')
            res[f'{k}_{name}']=dict(km=km,mean=mean,p90=p90,path=[(int(a),int(b)) for a,b in p[::5]],cost=float(dist[r,q]/len(p)))
    json.dump(res,open('paths.json','w'))
