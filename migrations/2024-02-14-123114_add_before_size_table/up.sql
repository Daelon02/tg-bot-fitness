CREATE TABLE before_sizes (
   id UUID PRIMARY KEY,
   chest INT,
   waist INT,
   hips INT,
   hand_biceps INT,
   leg_biceps INT,
   calf INT,
   FOREIGN KEY (id) REFERENCES users (id)
);
