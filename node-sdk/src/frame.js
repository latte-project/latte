class RequestMessage {
  asString() {
    return JSON.stringify(this.toFrame());
  }
}

class Get extends RequestMessage {
  constructor(objectRef) {
    this.objectRef = objectRef;
  }

  toFrame() {
    return {
      Get: {
        object_ref: this.objectRef
      }
    };
  }
}

class Set_ extends RequestMessage {
  constructor(objectRef, latteObject) {
    this.objectRef = objectRef;
    this.latteObject = latteObject;
  }

  toFrame() {
    return {
      Set: {
        object_ref: this.objectRef, 
        latte_object: this.latteObject
      }
    };
  }
}

// TODO: Register, Invoke