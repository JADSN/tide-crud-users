# Staging image (busybox:musl)
# FROM busybox:musl
# ENV PIPELINE staging

# Production image (scratch)
FROM scratch
ENV PIPELINE release
COPY ./dist/my-app /
EXPOSE 8000
ENTRYPOINT ["/my-app"]
