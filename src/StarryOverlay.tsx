import React, { useMemo } from "react";
import styles from "./StarryOverlay.module.css";

interface StarryOverlayProps {
  show: boolean;
}

function randomVictoryMessage() {
  const messages = [
    "Nice",
    "Perfect︎",
    "Lovely",
    "Brilliant",
    "Wonderful",
    "Fantastic",
  ];
  const message = messages[Math.floor(Math.random() * messages.length)];
  return message;
}

const StarryOverlay: React.FC<StarryOverlayProps> = ({ show }) => {
  const stars = useMemo(() => {
    return Array.from({ length: 100 }).map((_, i) => ({
      id: i,
      left: `${Math.random() * 100}%`,
      top: `${Math.random() * 100}%`,
      delay: `${Math.random() * 3}s`,
      duration: `${2 + Math.random() * 3}s`,
      size: `${1 + Math.random() * 2}px`,
    }));
  }, []);
  const message = randomVictoryMessage();

  if (!show) {
    return null;
  }

  return (
    <div className={styles.overlay}>
      <div className={styles.content}>
        <h1 className={styles.title}>{message}</h1>
      </div>
      {stars.map((star) => (
        <div
          key={star.id}
          className={styles.star}
          style={{
            left: star.left,
            top: star.top,
            width: star.size,
            height: star.size,
            animationDelay: star.delay,
            animationDuration: star.duration,
          }}
        />
      ))}
    </div>
  );
};

export default StarryOverlay;
