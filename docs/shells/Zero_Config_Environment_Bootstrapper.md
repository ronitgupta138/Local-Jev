# Zero-Configuration JDK Environment Discovery

## 1. Automated Detection
- Probes /usr/lib/jvm/, /opt/, and ~/.sdkman/candidates/java/.
- Extracts vendor string (Temurin, Corretto, GraalVM) and major release version in sub-5ms.
