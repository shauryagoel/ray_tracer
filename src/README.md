# Introduction

Summarize important concepts from the book.

## Chapter 1

Each tuple has 4 components- *x*, *y*, *z* and *w*.

$w = 1$ for a point

$w = 0$ for a vector

Thus, subtracting two points gives a vector.

## Chapter 4

We can create transformation matrices to **translate**, **scale** and **rotate** points and vectors.
It is done by multiplying the transformation matrix with the point or the vector-
$$ \text{transformation\_matrix} * \text{vector} $$

We can also chain transformations matrices and result would be the same as if they were applied to the vector one by one-
$$ \text{transformation\_matrix}_2 * \text{transformation\_matrix}_1 * \text{vector} $$

### Translate

It moves a point.
Vectors are not affected as it is just an arrow. Translating does not change its length or direction.

$$
translation(x, y, z) =
\begin{bmatrix}
1 & 0 & 0 & x \\
0 & 1 & 0 & y \\
0 & 0 & 1 & z \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

### Scaling

Move a point by multiplying it by a value.
For vectors, this transformation changes their length.

*Reflection* can be done by this transformation by multiplying the desired component by -1.

$$
scaling(x, y, z) =
\begin{bmatrix}
x & 0 & 0 & 0 \\
0 & y & 0 & 0 \\
0 & 0 & z & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

### Rotation

We can rotate a point or a point around *x*, *y* or *z* axis by a particular value of radians.
Point moves towards the direction of the fingers in the left-hand thumb rule, thumb points towards the positive side of the axis of the rotation.

$$ radians(deg) = \frac{deg}{180} \pi $$

$$
rotation_x(r) =
\begin{bmatrix}
1 & 0 & 0 & 0 \\
0 & \cos(r) & -\sin(r) & 0 \\
0 & \sin(r) & \cos(r) & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

$$
rotation_y(r) =
\begin{bmatrix}
\cos(r) & 0 & \sin(r) & 0 \\
0 & 1 & 0 & 0 \\
-\sin(r) & 0 & \cos(r) & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

$$
rotation_z(r) =
\begin{bmatrix}
\cos(r) & -\sin(r) & 0 & 0 \\
\sin(r) & cos(r) & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

### Shearing

Shearing transformation changes each component of a tuple in proportion to the other components. So, the *x* component changes in proportion to *y* and *z*, *y* changes in proportion to *x* and *z*, and *z* changes in proportion to *x* and *y*.

$$
shearing(x_y, x_z, y_x, y_z, z_x, z_y) =
\begin{bmatrix}
1 & x_y & x_z & 0 \\
y_x & 1 & y_z & 0 \\
z_x & z_y & 1 & 0 \\
0 & 0 & 0 & 1
\end{bmatrix}
$$

## Chapter 5

**Ray casting** is the process of creating a ray, or line, and finding the intersections of that ray with the objects in a scene.

**Hit** will always be the intersection of ray with the lowest non-negative *t* (timestamp) value.

### Transforming ray and spheres

Until now our sphere is always at origin and is of radius 1. To compute intersections of other sphere locations and sizes, we need to change our intersection algorithm. To prevent doing so, we can transform the ray itself assuming the sphere is the default one.
The transformations which moves the default sphere to interesting locations and sizes, we can apply the inverse of that to both of the ray's components.

Another way to think it is that transformations converts points between two coordinate systems-

1) At the scene level, everything is in the *world space coordinates* relative to the overall world
2) At the object level, everything is in *object space coordinates* relative to the object itself. Like default sphere being at origin and of radius 1.

Now, to intersect a ray in world space with a sphere in object space, we just convert the ray's origin and direction to that same object space.

## Chapter 6

When *P* is the point where our ray intersects an object, there are four vectors defined-

1) *E* is the *eye vector* pointing from *P* to the origin of the ray (usually where the eye exists that is looking at the scene).
2) *L* is the *light vector* pointing from *P* to the position of the light source.
3) *N* is the *surface normal*, vector that is perpendicular to the surface at *P*.
4) *R* is the *reflection vector* pointing in the direction that incoming light would bounce or reflect.

**Surface normal** is a vector perpendicular to the surface at a given point. It is a normalized vector.
