/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::XRSpaceBinding;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::reflector::reflect_dom_object;
use crate::dom::bindings::root::{Dom, DomRoot, MutNullableDom};
use crate::dom::eventtarget::EventTarget;
use crate::dom::globalscope::GlobalScope;
use crate::dom::xrinputsource::XRInputSource;
use crate::dom::xrreferencespace::XRReferenceSpace;
use crate::dom::xrsession::XRSession;
use dom_struct::dom_struct;
use euclid::{RigidTransform3D, Rotation3D, Vector3D};
use webvr_traits::{WebVRFrameData, WebVRPose};

#[dom_struct]
pub struct XRSpace {
    eventtarget: EventTarget,
    session: Dom<XRSession>,
    is_viewerspace: bool,
    input_source: MutNullableDom<XRInputSource>,
}

impl XRSpace {
    pub fn new_inherited(session: &XRSession) -> XRSpace {
        XRSpace {
            eventtarget: EventTarget::new_inherited(),
            session: Dom::from_ref(session),
            is_viewerspace: false,
            input_source: Default::default(),
        }
    }

    fn new_viewerspace_inner(session: &XRSession) -> XRSpace {
        XRSpace {
            eventtarget: EventTarget::new_inherited(),
            session: Dom::from_ref(session),
            is_viewerspace: true,
            input_source: Default::default(),
        }
    }

    pub fn new_viewerspace(global: &GlobalScope, session: &XRSession) -> DomRoot<XRSpace> {
        reflect_dom_object(
            Box::new(XRSpace::new_viewerspace_inner(session)),
            global,
            XRSpaceBinding::Wrap,
        )
    }

    fn new_inputspace_inner(session: &XRSession, input: &XRInputSource) -> XRSpace {
        XRSpace {
            eventtarget: EventTarget::new_inherited(),
            session: Dom::from_ref(session),
            is_viewerspace: false,
            input_source: MutNullableDom::new(Some(input)),
        }
    }

    pub fn new_inputspace(
        global: &GlobalScope,
        session: &XRSession,
        input: &XRInputSource,
    ) -> DomRoot<XRSpace> {
        reflect_dom_object(
            Box::new(XRSpace::new_inputspace_inner(session, input)),
            global,
            XRSpaceBinding::Wrap,
        )
    }
}

impl XRSpace {
    /// Gets pose represented by this space
    ///
    /// The reference origin used is common between all
    /// get_pose calls for spaces from the same device, so this can be used to compare
    /// with other spaces
    pub fn get_pose(&self, base_pose: &WebVRFrameData) -> RigidTransform3D<f64> {
        if let Some(reference) = self.downcast::<XRReferenceSpace>() {
            reference.get_pose(base_pose)
        } else if self.is_viewerspace {
            XRSpace::pose_to_transform(&base_pose.pose)
        } else if let Some(source) = self.input_source.get() {
            XRSpace::pose_to_transform(&source.pose())
        } else {
            unreachable!()
        }
    }

    pub fn pose_to_transform(pose: &WebVRPose) -> RigidTransform3D<f64> {
        let pos = pose.position.unwrap_or([0., 0., 0.]);
        let translation = Vector3D::new(pos[0] as f64, pos[1] as f64, pos[2] as f64);
        let orient = pose.orientation.unwrap_or([0., 0., 0., 0.]);
        let rotation = Rotation3D::quaternion(
            orient[0] as f64,
            orient[1] as f64,
            orient[2] as f64,
            orient[3] as f64,
        )
        .normalize();
        RigidTransform3D::new(rotation, translation)
    }

    pub fn session(&self) -> &XRSession {
        &self.session
    }
}
