import json, numpy as np, sys
sys.path.insert(0,'.')
F=json.load(open('features.json'))
names={'A':'A-negev-arava','B':'B-hisma-rum','C':'C-sinai','D':'D-harrat-azraq'}
REED={'A':(62.8,51.1),'B':(77.5,29.5),'C':(14.2,7.9),'D':(75.5,85.6)}
TRI={'A':[(56.9,3.8),(55.9,5.8),(56.5,10.3)],'B':[(77.5,110.5),(77.0,116.2),(76.0,120.8)],
     'C':[(23.6,37.9),(25.3,45.4),(27.8,50.9)],'D':[(75.1,55.9),(75.1,64.6),(75.5,72.1)]}
HIGH={'A':[('crater (Negev erosion cirque)',None),('Edom plateau massif',(94.9,40.7))],'B':[('sandstone massif',(53.6,119.3))],'C':[('granite massif',(49.1,26.3))],'D':[('basalt rise',(26.4,86.4))]}
def km(r,q): return (q+0.5)*0.3,(500-r-0.5)*0.3
for k in 'ABCD':
    e=np.load(f'{k}_e.npy'); s=np.load(f'{k}_s.npy')
    road=np.array(F[k]['path'])
    rows,cols=np.where((s>15)&(np.abs(e)>=0.5))
    P=np.array([km(r,q) for r,q in zip(rows,cols)]); esc=None
    if len(P):
        m=(P[:,1]>15)&(P[:,1]<135)&(P[:,0]>5)&(P[:,0]<145)
        Q=P[m]
        if len(Q):
            d=np.sqrt(((Q[:,None,:]-road[None,:,:])**2).sum(-1)).min(1)
            # nearest cell among those within 8 km, closest to mid-road
            sel=Q[d<8]
            if len(sel): esc=sel[np.argmin(np.abs(sel[:,1]-75))]
    marks=[{"n":1,"label":"Dry Meridian road","points":road.tolist()}]
    if esc is not None: marks.append({"n":2,"label":f"escarpment (slope>15 deg)","points":[[round(float(esc[0]),1),round(float(esc[1]),1)]]})
    marks.append({"n":3,"label":"Reedbank","points":[list(REED[k])]})
    marks.append({"n":4,"label":"Three Crossings","points":[list(p) for p in TRI[k]]})
    for lab,pt in HIGH[k]:
        if pt is None: continue
        marks.append({"n":5,"label":lab,"points":[list(pt)]})
    if k=='A': marks.append({"n":5,"label":"crater","points":[[56.0,107.0]]})
    json.dump(marks,open(f'docs/canon/terrain_candidates/{names[k]}.marks.json','w'),indent=1)
    print(k,esc)
