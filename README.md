# Roaring-rs-python-binding

Using rye to control python environment
```bash
rye sync 
```

Using maturin to compile project 

```bash
maturin develop
```

Create RoaringBitmap and try some methods

```bash
rye run ipython
```

```python
In [1]: import roaring

In [2]: from roaring import RoaringBitmap

In [3]: a = RoaringBitmap()

In [4]: a
Out[4]: RoaringBitmap<[]>

In [5]: a.insert(1)
Out[5]: True

In [6]: a.insert(1)
Out[6]: False

In [7]: a.insert(2)
Out[7]: True

In [8]: a
Out[8]: RoaringBitmap<[1, 2]>

In [9]: b = RoaringBitmap()

In [10]: a.remove(2)
Out[10]: True

In [11]: a.is_disjoint(b)
Out[11]: True
```
