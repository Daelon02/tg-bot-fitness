CREATE TABLE sizes (
   id UUID PRIMARY KEY,
   user_id UUID,
   chest INT,
   waist INT,
   hips INT,
   hand_biceps INT,
   leg_biceps INT,
   calf INT,
   FOREIGN KEY (user_id) REFERENCES users (id)
);
