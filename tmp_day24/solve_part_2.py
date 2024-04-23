import numpy as np

import sys

def determinant(A):
  if A.shape == (1,1):
    return A[0][0]
  det = 0
  sgn = 1
  A_rem = np.delete(A, 0, axis=1)
  for i in range(A.shape[0]):
    det += sgn * A[i][0] * determinant(np.delete(A_rem, i, axis=0))
    sgn = -sgn
  return det

def adjugate(A):
  det = determinant(A)
  assert det != 0
  M = lambda i,j: np.delete(np.delete(A, i, axis=0), j, axis=1)
  n = A.shape[0]
  a_inv = np.array([[(-1)**(i+j) * determinant(M(i,j)) for j in range(n)] for i in range(n) ])
  return np.transpose(a_inv)

def solve(filename):
  lines = [l.strip() for l in open(filename).readlines()]
  vecs = []
  for line in lines:
    tmp = [t.strip() for t in line.split('@')]
    p_str = tmp[0]
    v_str = tmp[1]
    p = np.array([0] + [int(t.strip()) for t in p_str.split(',')], dtype=np.dtype('object'))
    v = np.array([1] + [int(t.strip()) for t in v_str.split(',')], dtype=np.dtype('object'))
    vecs.append((p,v))

  some_intersection_pts = []
  for i in range(2):
    t1,t2,t3 = vecs[i:i+3]
    p1,v1=t1
    p2,v2=t2
    p3,v3=t3
    A = np.column_stack((v1, p2+v2-p1, p2-p1, -v3))
    b = p3 - p1

    A_adj = adjugate(A)
    A_det = determinant(A)
    print(f'det: {A_det}')
    assert A_det != 0

    c_td = np.matmul(A_adj, b)
    print(f'c_td: {c_td}')
    isct_td = p3 * A_det + c_td[3] * v3
    print(f'isct_td: {isct_td}')
    some_intersection_pts.append((isct_td, A_det))

  q1d, q2d = some_intersection_pts[:2]
  q1,d1 = q1d
  q2,d2 = q2d
  print(f'q1/d1={q1/d1}')
  print(f'q2/d2={q2/d2}')
  # line(th): q1 + th * (q2-q1)
  t0 = q1[0]/d1 / (q2/d2-q1/d1)[0]
  p0 = q1/d1 - t0 * (q2/d2-q1/d1)
  p0s = p0.sum()

  # answer: 808107741406756
  print(f'the starting point is {p0}')
  print(f'the starting point sum is {p0s}')

if __name__ == '__main__':
  solve(sys.argv[1])
